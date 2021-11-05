use std::fmt::{write, Arguments, Result, Write};

pub struct Formatter<'a> {
    indent_level: usize,
    indent_size: usize,
    prev_char: char,
    buf: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            indent_level: 0,
            indent_size: 4,
            prev_char: ' ',
            buf,
        }
    }

    pub fn indent_size(&mut self, indent_size: usize) {
        self.indent_size = indent_size;
    }

    pub fn indent_level_up(&mut self) {
        self.indent_level += 1;
    }

    pub fn indent_level_down(&mut self) {
        self.indent_level -= 1;
    }
}

impl<'a> Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        for char in s.chars() {
            self.write_char(char)?;
        }
        Result::Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        if c != '\n' && self.prev_char == '\n' {
            self.buf
                .write_str(&*" ".repeat(self.indent_level * self.indent_size))?;
        };
        self.prev_char = c;
        self.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        write(self, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::middle_level_ir::format::Formatter;
    use crate::middle_level_ir::ml_decl::{MLFun, MLFunBody};
    use crate::middle_level_ir::ml_expr::{MLExpr, MLLiteral};
    use crate::middle_level_ir::ml_node::MLNode;
    use crate::middle_level_ir::ml_stmt::MLStmt;
    use crate::middle_level_ir::ml_type::{MLPrimitiveType, MLValueType};

    #[test]
    fn test_format_indent_level() {
        let fun = MLFun {
            modifiers: vec![],
            name: "f".to_string(),
            arg_defs: vec![],
            return_type: MLValueType::Primitive(MLPrimitiveType::Noting),
            body: Some(MLFunBody {
                body: vec![MLStmt::Expr(MLExpr::Literal(MLLiteral::Integer { value: "0".to_string(), type_: MLValueType::Primitive(MLPrimitiveType::Int8) }))]
            })
        };
        let mut buf = String::new();
        let mut formatter = Formatter::new(&mut buf);
        formatter.indent_size(2);
        let _ = fun.fmt(&mut formatter);
        assert_eq!(buf, String::from("fun f():Noting {\n  0;\n};"));
    }
}
