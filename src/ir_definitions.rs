use lang_c::ast::{Identifier, TypeSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnitIr {
    pub name: String,
    pub functions: Vec<FnIr>,
    pub type_definitions: Vec<TypeDefintionsIr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnIr {
    pub ident: Identifier,
    pub return_type: TypeSpecifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefintionsIr {}
