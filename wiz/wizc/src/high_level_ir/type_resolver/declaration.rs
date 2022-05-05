use crate::high_level_ir::type_resolver::context::ResolverStruct;
use crate::high_level_ir::type_resolver::namespace::Namespace;
use crate::high_level_ir::typed_type::TypedType;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclarationItem {
    Namespace(Namespace),
    Type(ResolverStruct),
    Value((Vec<String> /* namespace */, TypedType)),
}
