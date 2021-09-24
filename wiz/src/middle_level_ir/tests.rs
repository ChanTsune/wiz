use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::Ast2HLIR;
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::HLIR2MLIR;
use crate::parser::parser::parse_from_string;

#[test]
fn test_empty() {
    let source = "";

    let ast = parse_from_string(String::from(source)).unwrap();

    let mut ast2hlir = Ast2HLIR::new();

    let mut file = ast2hlir.file(ast);
    file.name = String::from("test");

    let mut resolver = TypeResolver::new();
    let _ = resolver.detect_type(&file);
    let _ = resolver.preload_file(file.clone());
    let hl_file = resolver.file(file).unwrap();

    let mut hlir2mlir = HLIR2MLIR::new();

    let f = hlir2mlir.file(hl_file);

    assert_eq!(
        f,
        MLFile {
            name: "test".to_string(),
            body: vec![]
        }
    );
}
