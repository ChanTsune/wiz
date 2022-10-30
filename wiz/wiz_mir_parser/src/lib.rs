use crate::error::PResult;
use crate::lexer::parse_token_trees;
use wiz_mir_syntax::token::TokenStream;

pub mod error;
pub mod lexer;
pub mod parser;

pub fn maybe_file_to_parser() {}

pub fn maybe_file_to_stream() {}

pub fn maybe_str_to_stream(s: &str) -> PResult<TokenStream> {
    parse_token_trees(s, 0)
}

pub fn stream_to_parser() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
