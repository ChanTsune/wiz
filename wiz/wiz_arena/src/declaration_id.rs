#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DeclarationId(usize);

impl DeclarationId {
    pub const DUMMY: Self = Self::new(usize::MAX);
    pub const ROOT: Self = Self::new(0);
    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct DeclarationIdGenerator {
    latest: usize,
}

impl DeclarationIdGenerator {
    pub fn new(initial: usize) -> Self {
        Self { latest: initial }
    }

    pub fn generate(&mut self) -> DeclarationId {
        self.latest += 1;
        DeclarationId::new(self.latest)
    }
}
