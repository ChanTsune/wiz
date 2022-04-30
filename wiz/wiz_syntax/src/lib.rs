pub mod syntax;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub const MIN: Self = Self(usize::MIN);
    pub const MAX: Self = Self(usize::MAX);
    pub const ROOT: Self = Self::new(0);
    pub const DUMMY: Self = Self::MAX;

    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct NodeIdGenerator {
    latest: usize,
}

impl NodeIdGenerator {
    pub fn new(initial: NodeId) -> Self {
        Self { latest: initial.0 }
    }

    pub fn next(&mut self) -> NodeId {
        self.latest += 1;
        NodeId::new(self.latest)
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeId, NodeIdGenerator};

    #[test]
    fn test_generate() {
        let mut node_id_generator = NodeIdGenerator::new(NodeId::ROOT);
        assert_eq!(node_id_generator.next(), NodeId::new(1));
        assert_eq!(node_id_generator.next(), NodeId::new(2));
        assert_eq!(node_id_generator.next(), NodeId::new(3));
    }
}
