use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::Ast2HLIR;
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::HLIR2MLIR;
use crate::parser::parser::parse_from_string;
use crate::middle_level_ir::ml_decl::{MLDecl, MLFun, MLFunBody};
use crate::middle_level_ir::ml_type::MLValueType;
use crate::middle_level_ir::ml_stmt::MLStmt;
use crate::middle_level_ir::ml_expr::{MLExpr, MLReturn, MLLiteral};

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

#[test]
fn test_return_integer_literal() {
    let source = r"
    fun integer(): Int64 {
        return 1
    }
    ";

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
            body: vec![MLDecl::Fun(MLFun {
                modifiers: vec![],
                name: "integer".to_string(),
                arg_defs: vec![],
                return_type: MLValueType::Primitive(String::from("Int64")),
                body: Some(MLFunBody {
                    body: vec![MLStmt::Expr(MLExpr::Return(MLReturn {
                        value: Some(Box::new(MLExpr::Literal(MLLiteral::Integer {
                            value: "1".to_string(),
                            type_: MLValueType::Primitive(String::from("Int64"))
                        }))),
                        type_: MLValueType::Primitive(String::from("Int64"))
                    }))]
                })
            })]
        }
    );
}
