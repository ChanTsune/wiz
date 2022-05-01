#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TypeId(usize);

impl TypeId {
    const DUMMY: Self = Self::new(usize::MAX);
    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
struct TypeIdGenerator {
    latest: usize,
}

impl TypeIdGenerator {
    pub fn new(initial: usize) -> Self {
        Self { latest: initial }
    }

    pub fn next(&mut self) -> TypeId {
        self.latest += 1;
        TypeId::new(self.latest)
    }
}
