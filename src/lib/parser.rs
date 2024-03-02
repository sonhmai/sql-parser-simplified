/*
Parser parses tokens into AST
 */

use crate::lib::ast::statements::Statement;

pub struct Parser {}

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
}

pub enum ParserError {
    ParserError(String),
    RecursionLimitExceeded,
}
