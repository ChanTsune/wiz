use std::fmt::{Write, Arguments, Result};

pub struct Formatter<'a> {
    indent_level: usize,
    indent_size: usize,
    prev_char: char,
    buf: &'a mut (dyn Write + 'a),
}

impl <'a> Formatter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self { indent_level: 0, indent_size: 4, prev_char: ' ', buf }
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

impl <'a> Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        for char in s.chars() {
            self.write_char(char)?;
        }
        Result::Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        if c == '\n' && self.prev_char != '\n' {
            self.buf.write_str(&*" ".repeat(self.indent_level * self.indent_size))?;
        };
        self.prev_char = c;
        self.buf.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> Result {
        todo!()
    }
}