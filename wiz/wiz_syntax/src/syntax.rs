use crate::syntax::trivia::Trivia;
use std::fmt::Debug;

pub mod annotation;
pub mod block;
pub mod declaration;
pub mod expression;
pub mod file;
mod list;
pub mod literal;
pub mod modifier;
pub mod name_space;
pub mod statement;
pub mod token;
pub mod trivia;
pub mod type_name;

pub trait Syntax: Debug + Eq + PartialEq + Clone {
    fn with_leading_trivia(self, trivia: Trivia) -> Self;
    fn with_trailing_trivia(self, trivia: Trivia) -> Self;
    fn span(&self) -> Location {
        Location::default()
    }
}

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

pub fn get_line_offset(s: &str, location: &Location) -> usize {
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

#[test]
fn test_get_location() {
    let location = Location::new(1, 0);
    assert_eq!(location.offset(), 1);
    assert_eq!(location.line(), 0);
}

#[cfg(test)]
mod get_line_offset {
    use super::*;

    #[test]
    fn one_line() {
        let location = Location::new(1, 1);
        assert_eq!(get_line_offset("a\n2", &location), 1);
    }

    #[test]
    fn one_line_second() {
        let location = Location::new(3, 2);
        assert_eq!(get_line_offset("a\n2", &location), 1);
    }

    #[test]
    fn test_three_lines() {
        let location = Location::new(7, 4);
        assert_eq!(get_line_offset("a\n2\n\n33", &location), 2);
    }
}
