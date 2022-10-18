use std::collections::HashMap;
use wiz_hir::typed_type::TypedType;

pub(crate) type TyEnv<'a> = TypeEnvironment<'a, TypedType>;

pub struct TypeEnvironment<'a, Type>
where
    Type: Clone,
{
    parent_env: Option<&'a Self>,
    current_env: HashMap<String, Type>,
}

impl<'a, Type> TypeEnvironment<'a, Type>
where
    Type: Clone,
{
    pub fn root() -> Self {
        Self {
            parent_env: None,
            current_env: Default::default(),
        }
    }

    pub(crate) fn new(parent: &'a Self) -> Self {
        Self {
            parent_env: Some(parent),
            current_env: Default::default(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Type> {
        self.current_env
            .get(name)
            .cloned()
            .or_else(|| self.parent_env?.lookup(name))
    }

    pub fn extend(&mut self, name: &str, ty: Type) {
        self.current_env.insert(name.to_owned(), ty);
    }

    pub fn new_scope<T, F: FnOnce(Self) -> T>(&'a self, f: F) -> T {
        f(Self::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::TypeEnvironment;
    use wiz_hir::typed_type::TypedType;

    #[test]
    fn type_env() {
        let mut ty_env = TypeEnvironment::<TypedType>::root();
        ty_env.extend("a", TypedType::string_ref());
        ty_env.extend("b", TypedType::usize());
        assert_eq!(ty_env.lookup("a"), Some(TypedType::string_ref()));

        ty_env.new_scope(|mut ty_env| {
            ty_env.extend("a", TypedType::noting());
            assert_eq!(ty_env.lookup("a"), Some(TypedType::noting()));
            assert_eq!(ty_env.lookup("b"), Some(TypedType::usize()));
        });
        assert_eq!(ty_env.lookup("a"), Some(TypedType::string_ref()));
    }
}
