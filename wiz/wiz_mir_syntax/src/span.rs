pub const DUMMY_SPAN: Span = Span {
    index: 0,
    length: 0,
};

/// Token position and length in source.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Span {
    /// Token index.
    index: usize,
    /// Token length
    length: usize,
}

impl Span {
    pub fn new(index: usize, length: usize) -> Self {
        Self { index, length }
    }

    pub fn to(&self, end: &Self) -> Self {
        Self::new(self.index, (end.index - self.index) + end.length)
    }
}

#[cfg(test)]
mod tests {
    use crate::span::Span;

    #[test]
    fn test_span_to() {
        let start = Span::new(0, 1);
        let end = Span::new(1, 1);
        assert_eq!(start.to(&end), Span::new(0, 2));

        let start = Span::new(0, 1);
        let end = Span::new(4, 5);
        assert_eq!(start.to(&end), Span::new(0, 9));
    }
}
