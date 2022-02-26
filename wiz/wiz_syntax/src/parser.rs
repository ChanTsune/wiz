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
