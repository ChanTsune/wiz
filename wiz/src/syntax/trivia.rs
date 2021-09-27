#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TriviaPiece {
    /// A space ' ' character.
    Spaces(i64),
    /// A tab '\t' character.
    Tabs(i64),
    /// A vertical tab '\v' character.
    VerticalTabs(i64),
    /// A form-feed 'f' character.
    FormFeeds(i64),
    /// A newline '\n' character.
    Newlines(i64),
    /// A newline '\r' character.
    CarriageReturns(i64),
    /// A newline consists of contiguous '\r' and '\n' characters.
    CarriageReturnLineFeeds(i64),
    /// A developer line comment, starting with '//'
    LineComment(String),
    /// A developer block comment, starting with '/*' and ending with '*/'.
    BlockComment(String),
    /// A documentation line comment, starting with '///'.
    DocLineComment(String),
    /// A documentation block comment, starting with '/**' and ending with '*/'.
    DocBlockComment(String),
    /// Any skipped garbage text.
    GarbageText(String),
}
impl ToString for TriviaPiece {
    fn to_string(&self) -> String {
        match self {
            TriviaPiece::Spaces(i) => String::from(' ').repeat(i.clone() as usize),
            TriviaPiece::Tabs(i) => String::from('\t').repeat(i.clone() as usize),
            TriviaPiece::VerticalTabs(i) => String::from('\x11').repeat(i.clone() as usize),
            TriviaPiece::FormFeeds(i) => String::from('\x12').repeat(i.clone() as usize),
            TriviaPiece::Newlines(i) => String::from('\n').repeat(i.clone() as usize),
            TriviaPiece::CarriageReturns(i) => String::from('\r').repeat(i.clone() as usize),
            TriviaPiece::CarriageReturnLineFeeds(i) => {
                String::from("\r\n").repeat(i.clone() as usize)
            }
            TriviaPiece::LineComment(s) => s.clone(),
            TriviaPiece::BlockComment(s) => s.clone(),
            TriviaPiece::DocLineComment(s) => s.clone(),
            TriviaPiece::DocBlockComment(s) => s.clone(),
            TriviaPiece::GarbageText(s) => s.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Trivia {
    peaces: Vec<TriviaPiece>,
}
