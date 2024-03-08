use crate::lib::ast::expression::Expr;
use crate::lib::ast::expression::Ident;
use crate::lib::ast::query::Query;

/// A top level statement (SELECT, INSERT, etc.)
#[derive(Debug)]
pub enum Statement {
    Query {
        limit: Option<Expr>
    },
    Insert {
        table_name: String,
        columns: Vec<Ident>,
        /// A SQL Query specifying what to insert
        source: Option<Box<Query>>
    },
}
