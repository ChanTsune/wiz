mod string_reader;
mod token_stream_builder;
pub mod token_trees;

use crate::error::PResult;
use crate::lexer::string_reader::StringReader;
use crate::lexer::token_trees::TokenTreeReader;
use wiz_mir_syntax::token::TokenStream;

pub fn parse_token_trees(src: &str, start_position: usize) -> PResult<TokenStream> {
    let mut tt_reader = TokenTreeReader::from(StringReader::new(src, start_position, src.len()));
    tt_reader.parse_all_token_trees()
}
