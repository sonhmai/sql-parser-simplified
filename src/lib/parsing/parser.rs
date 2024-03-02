/*
Parser parses tokens into AST
 */

use crate::lib::ast::statements::Statement;
use crate::lib::dialect::Dialect;

pub struct Parser {}

impl Parser {
    pub fn new(dialect: &dyn Dialect) -> Self {
        Self {}
    }
    pub fn parse_statement(&mut self, sqlstr: &str) -> Result<Statement, ParserError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ParserError {
    ParserError(String),
    RecursionLimitExceeded,
}
