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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Trivia {
    peaces: Vec<TriviaPiece>,
}
