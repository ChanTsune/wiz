use crate::ast::decl::Decl;
use crate::ast::stmt::Stmt;
use crate::ast::node::Node;
use std::fmt;

#[derive(fmt::Debug)]
struct DeclStmt {
    decl: dyn Decl
}

impl Node for DeclStmt {

}

impl Stmt for DeclStmt {

}