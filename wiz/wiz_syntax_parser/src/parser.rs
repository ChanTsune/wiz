use nom_locate::LocatedSpan;

pub mod error;
pub mod result;
#[cfg(test)]
mod tests;
pub mod wiz;

pub type Span<'a> = LocatedSpan<&'a str>;
