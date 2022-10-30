use wiz_mir_syntax::span as syntax_span;
use wiz_mir_syntax::token as syntax_token;

pub struct StringReader<'a> {
    /// Start reading src at this position
    start_position: usize,
    /// Current reading position
    position: usize,
    /// Stop reading src at this position
    end_src_index: usize,
    /// Target src
    src: &'a str,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str, start_position: usize, end_src_index: usize) -> Self {
        Self {
            start_position,
            position: start_position,
            end_src_index,
            src,
        }
    }

    pub fn next_token(&mut self) -> (syntax_token::Spacing, syntax_token::Token) {
        let mut spacing = syntax_token::Spacing::Joint;
        loop {
            let text = &self.src[self.position..];
            if text.is_empty() {
                let span = syntax_span::Span::new(self.position, 0);
                return (
                    spacing,
                    syntax_token::Token::new(syntax_token::TokenKind::Eof, span),
                );
            }

            let token = wiz_lexar::first_token(text);
            let start = self.position;
            // Update current position
            self.position += token.len;

            match self.cook_lexer_token(token.kind) {
                Some(kind) => {
                    let span = syntax_span::Span::new(start, self.position - start);
                    return (spacing, syntax_token::Token::new(kind, span));
                }
                None => spacing = syntax_token::Spacing::Alone,
            }
        }
    }

    fn cook_lexer_token(
        &mut self,
        lexer_token_kind: wiz_lexar::TokenKind,
    ) -> Option<syntax_token::TokenKind> {
        Some(match lexer_token_kind {
            wiz_lexar::TokenKind::LineComment { doc_style } => {
                // Skip non doc style comments
                let doc_style = doc_style?;
                self.cook_doc_comment(syntax_token::CommentKind::Line, doc_style)
            }
            wiz_lexar::TokenKind::BlockComment {
                doc_style,
                terminated,
            } => {
                if !terminated {
                    panic!("comment block not matched!")
                };
                // Skip non doc style comments
                let doc_style = doc_style?;
                self.cook_doc_comment(syntax_token::CommentKind::Block, doc_style)
            }
            wiz_lexar::TokenKind::Whitespace => return None,
            wiz_lexar::TokenKind::Identifier => syntax_token::TokenKind::Ident(false),
            wiz_lexar::TokenKind::RawIdentifier { err } => {
                if let Some(err) = err {
                    panic!("{:?}", err)
                };
                syntax_token::TokenKind::Ident(true)
            }
            wiz_lexar::TokenKind::UnknownPrefix => syntax_token::TokenKind::Ident(false),
            wiz_lexar::TokenKind::Literal { kind, suffix_start } => {
                syntax_token::TokenKind::Literal(syntax_token::Lit {
                    kind: self.cook_lexer_literal(kind),
                })
            }
            wiz_lexar::TokenKind::Lifetime { starts_with_number } => {
                syntax_token::TokenKind::Lifetime
            }
            wiz_lexar::TokenKind::Semi => syntax_token::TokenKind::Semi,
            wiz_lexar::TokenKind::Comma => syntax_token::TokenKind::Comma,
            wiz_lexar::TokenKind::Dot => syntax_token::TokenKind::Dot,
            wiz_lexar::TokenKind::OpenParen => {
                syntax_token::TokenKind::OpenDelim(syntax_token::DelimToken::Paren)
            }
            wiz_lexar::TokenKind::CloseParen => {
                syntax_token::TokenKind::CloseDelim(syntax_token::DelimToken::Paren)
            }
            wiz_lexar::TokenKind::OpenBrace => {
                syntax_token::TokenKind::OpenDelim(syntax_token::DelimToken::Brace)
            }
            wiz_lexar::TokenKind::CloseBrace => {
                syntax_token::TokenKind::CloseDelim(syntax_token::DelimToken::Brace)
            }
            wiz_lexar::TokenKind::OpenBracket => {
                syntax_token::TokenKind::OpenDelim(syntax_token::DelimToken::Bracket)
            }
            wiz_lexar::TokenKind::CloseBracket => {
                syntax_token::TokenKind::CloseDelim(syntax_token::DelimToken::Bracket)
            }
            wiz_lexar::TokenKind::At => syntax_token::TokenKind::At,
            wiz_lexar::TokenKind::Pound => syntax_token::TokenKind::Pound,
            wiz_lexar::TokenKind::Tilde => syntax_token::TokenKind::Tilde,
            wiz_lexar::TokenKind::Question => syntax_token::TokenKind::Question,
            wiz_lexar::TokenKind::Colon => syntax_token::TokenKind::Colon,
            wiz_lexar::TokenKind::Dollar => syntax_token::TokenKind::Dollar,
            wiz_lexar::TokenKind::Eq => syntax_token::TokenKind::Eq,
            wiz_lexar::TokenKind::Bang => syntax_token::TokenKind::Not,
            wiz_lexar::TokenKind::Lt => syntax_token::TokenKind::Lt,
            wiz_lexar::TokenKind::Gt => syntax_token::TokenKind::Gt,
            wiz_lexar::TokenKind::Minus => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Minus)
            }
            wiz_lexar::TokenKind::And => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::And)
            }
            wiz_lexar::TokenKind::Or => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Or)
            }
            wiz_lexar::TokenKind::Plus => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Plus)
            }
            wiz_lexar::TokenKind::Star => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Star)
            }
            wiz_lexar::TokenKind::Slash => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Slash)
            }
            wiz_lexar::TokenKind::Caret => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Caret)
            }
            wiz_lexar::TokenKind::Percent => {
                syntax_token::TokenKind::BinOp(syntax_token::BinOpToken::Percent)
            }
            wiz_lexar::TokenKind::Unknown => {
                panic!("Unknown token")
            }
        })
    }

    fn cook_doc_comment(
        &self,
        comment_kind: syntax_token::CommentKind,
        doc_style: wiz_lexar::DocStyle,
    ) -> syntax_token::TokenKind {
        let attr_style = match doc_style {
            wiz_lexar::DocStyle::Outer => syntax_token::AttrStyle::Outer,
            wiz_lexar::DocStyle::Inner => syntax_token::AttrStyle::Inner,
        };
        syntax_token::TokenKind::DocComment(comment_kind, attr_style)
    }

    fn cook_lexer_literal(&self, kind: wiz_lexar::LiteralKind) -> syntax_token::LitKind {
        match kind {
            wiz_lexar::LiteralKind::Int { .. } => syntax_token::LitKind::Integer,
            wiz_lexar::LiteralKind::Float { .. } => syntax_token::LitKind::Float,
            wiz_lexar::LiteralKind::Char { .. } => syntax_token::LitKind::Char,
            wiz_lexar::LiteralKind::Byte { .. } => syntax_token::LitKind::Byte,
            wiz_lexar::LiteralKind::Str { .. } => syntax_token::LitKind::Str,
            wiz_lexar::LiteralKind::ByteStr { .. } => syntax_token::LitKind::ByteStr,
            wiz_lexar::LiteralKind::RawStr { .. } => syntax_token::LitKind::StrRaw,
            wiz_lexar::LiteralKind::RawByteStr { .. } => syntax_token::LitKind::ByteStrRaw,
        }
    }
}
