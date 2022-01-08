use nom_locate::LocatedSpan;

pub mod error;
pub mod result;
#[cfg(test)]
mod tests;
pub mod wiz;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Location {
    offset: usize,
    line: u32,
}

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span<'a>) -> Self {
        Self {
            offset: span.location_offset(),
            line: span.location_line(),
        }
    }
}
