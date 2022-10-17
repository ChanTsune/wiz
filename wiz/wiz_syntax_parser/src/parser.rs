use nom_locate::LocatedSpan;

pub mod error;
#[cfg(test)]
mod tests;
pub mod wiz;

pub type Span<'a> = LocatedSpan<&'a str>;
