#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypingType<Id> {
    Unit,  // return no value
    Never, // never reach
    Primitive(TypingPrimitiveType),
    Struct(/* id */ Id),                                    // user defined type
    Array(Box<Self>, /* size */ usize),                     // array type
    Slice(Box<Self>),                                       // array reference
    Reference(Box<Self>),                                   // reference
    Pointer(Box<Self>),                                     // pointer
    Function(/* args */ Vec<Self>, /* return */ Box<Self>), // function
    TypeParam(Id),                                          // type parameter
    Namespace(Id),                                          // namespace
    Self_,                                                  // self type
    Alpha,                                                  // unresolved type
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypingPrimitiveType {
    Integer(u8),
    Size,
    UInteger(u8),
    USize,
    Float(u8),
    Bool,
    Str
}
