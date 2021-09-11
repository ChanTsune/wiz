use crate::middle_level_ir::ml_decl::MLDecl;
use std::fmt;
use crate::middle_level_ir::ml_node::MLNode;
use std::fmt::Write;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub struct MLFile {
    pub(crate) name: String,
    pub(crate) body: Vec<MLDecl>,
}

impl ToString for MLFile {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut formatter = crate::middle_level_ir::format::Formatter::new(&mut buf);
        self.fmt(&mut formatter);
        buf
    }
}

impl MLNode for MLFile {
    fn fmt(&self, f: &mut crate::middle_level_ir::format::Formatter) -> fmt::Result {
        f.indent_level_up();
        for stmt in self.body.iter() {
            stmt.fmt(f)?;
            f.write_char('\n')?;
        };
        f.indent_level_down();
        fmt::Result::Ok(())
    }
}
