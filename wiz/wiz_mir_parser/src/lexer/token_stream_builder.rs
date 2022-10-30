use wiz_mir_syntax::token::{Spacing, TokenStream, TokenTree, TreeAndSpacing};

#[derive(Default)]
pub(crate) struct TokenStreamBuilder {
    buf: Vec<TreeAndSpacing>,
}

impl TokenStreamBuilder {
    pub(crate) fn push(&mut self, (tree, joint): TreeAndSpacing) {
        // if let Some((TokenTree::Token(prev_token), Spacing::Joint)) = self.buf.last() {
        //     if let TokenTree::Token(token) = &tree {
        //         if let Some(glued) = prev_token.glue(token) {
        //             self.buf.pop();
        //             self.buf.push((TokenTree::Token(glued), joint));
        //             return;
        //         }
        //     }
        // }
        self.buf.push((tree, joint))
    }

    pub(crate) fn into_token_stream(self) -> TokenStream {
        TokenStream::new(self.buf)
    }
}
