use crate::error::PResult;
use crate::lexer::string_reader::StringReader;
use crate::lexer::token_stream_builder::TokenStreamBuilder;
use std::collections::HashMap;
use wiz_mir_syntax::span::Span;
use wiz_mir_syntax::token;
use wiz_mir_syntax::token::{
    DelimSpan, DelimToken, Spacing, Token, TokenKind, TokenStream, TokenTree, TreeAndSpacing,
};

pub struct UnmatchedBrace {
    pub expected_delim: token::DelimToken,
    pub found_delim: Option<token::DelimToken>,
    pub found_span: Span,
    pub unclosed_span: Option<Span>,
    pub candidate_span: Option<Span>,
}

pub struct TokenTreeReader<'a> {
    string_reader: StringReader<'a>,
    token: Token,
    /// Stack of open delimiters and their spans. Used for error message.
    open_braces: Vec<(token::DelimToken, Span)>,
    unmatched_braces: Vec<UnmatchedBrace>,
    /// The type and spans for all braces
    ///
    /// Used only for error recovery when arriving to EOF with mismatched braces.
    matching_delim_spans: Vec<(token::DelimToken, Span, Span)>,
    last_unclosed_found_span: Option<Span>,
    /// Collect empty block spans that might have been auto-inserted by editors.
    last_delim_empty_block_spans: HashMap<token::DelimToken, Span>,
    /// Collect the spans of braces (Open, Close). Used only
    /// for detecting if blocks are empty and only braces.
    matching_block_spans: Vec<(Span, Span)>,
}

impl<'a> TokenTreeReader<'a> {
    // Parse a stream of tokens into a list of `TokenTree`s, up to an `Eof`.
    pub(crate) fn parse_all_token_trees(&mut self) -> PResult<TokenStream> {
        let mut buf = TokenStreamBuilder::default();
        self.bump();
        while self.token.kind != token::TokenKind::Eof {
            buf.push(self.parse_token_trees()?);
        }
        Ok(buf.into_token_stream())
    }

    fn parse_token_trees(&mut self) -> PResult<TreeAndSpacing> {
        let prev_token = self.token.clone();
        let spacing = self.bump();
        match prev_token.kind {
            TokenKind::OpenDelim(d) => self.parse_token_until_close_delim(d, spacing),
            _ => Ok((TokenTree::Token(prev_token), spacing)),
        }
    }

    fn parse_token_until_close_delim(
        &mut self,
        open: DelimToken,
        spacing: Spacing,
    ) -> PResult<TreeAndSpacing> {
        let mut buf = TokenStreamBuilder::default();
        let mut spacing = spacing;
        loop {
            match self.token.kind {
                TokenKind::CloseDelim(_) => break,
                _ => buf.push((TokenTree::Token(self.token.clone()), spacing)),
            }
            spacing = self.bump();
        }
        Ok((
            TokenTree::Delimited(DelimSpan::dummy(), open, buf.into_token_stream()),
            spacing,
        ))
    }

    fn bump(&mut self) -> Spacing {
        let (spacing, token) = self.string_reader.next_token();
        self.token = token;
        spacing
    }
}

impl<'a> From<StringReader<'a>> for TokenTreeReader<'a> {
    fn from(string_reader: StringReader<'a>) -> Self {
        Self {
            string_reader,
            token: Token::dummy(),
            open_braces: vec![],
            unmatched_braces: vec![],
            matching_delim_spans: vec![],
            last_unclosed_found_span: None,
            last_delim_empty_block_spans: Default::default(),
            matching_block_spans: vec![],
        }
    }
}
