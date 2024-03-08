/*
Every node in the Syntax Tree is an Expression.
 */



use crate::lib::ast::expression::Ident;
use crate::lib::ast::expression::value::Value;

#[derive(Debug, Clone)]
pub enum Expr {
    /// Identifier: table name or column name. Value is identifier name without quote
    Identifier(Ident),
    /// A literal value, such as string, number, date or NULL
    Value(Value)
}