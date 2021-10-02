use nom_locate::LocatedSpan;

pub mod error;
pub mod result;
pub mod wiz;

pub(crate) type Span<'a> = LocatedSpan<&'a str>;

pub(crate) struct Location {
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
