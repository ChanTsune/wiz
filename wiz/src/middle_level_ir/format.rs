use std::fmt::Write;

pub struct Formatter<'a> {
    indent_level: u8,
    indent_size: u8,
    buf: &'a mut (dyn Write + 'a),
}

impl <'a> Formatter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self { indent_level: 0, indent_size: 4, buf }
    }

    pub fn indent_size(&mut self, indent_size: u8) {
        self.indent_size = indent_size;
    }

    pub fn indent_level_up(&mut self) {
        self.indent_level += 1;
    }

    pub fn indent_level_down(&mut self) {
        self.indent_level -= 1;
    }
}
