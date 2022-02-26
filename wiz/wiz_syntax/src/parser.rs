use nom_locate::LocatedSpan;

pub mod error;
pub mod result;
#[cfg(test)]
mod tests;
pub mod wiz;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Location {
    offset: usize,
    line: u32,
}

impl Location {
    pub fn new(offset: usize, line: u32) -> Self {
        Self { offset, line }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn line(&self) -> u32 {
        self.line
    }
}


impl<'a> From<&Span<'a>> for Location {
    fn from(span: &Span<'a>) -> Self {
        Self {
            offset: span.location_offset(),
            line: span.location_line(),
        }
    }
}


fn get_line_offset(s: &str, location: &Location) -> usize {
    let mut n = 1usize;
    let target_line = location.line() as usize;
    for (i, c) in s.char_indices() {
        if c == '\n' {
            n += 1;
            continue;
        };
        if n == target_line {
            return location.offset() - i;
        };
    }
    return 0;
}
