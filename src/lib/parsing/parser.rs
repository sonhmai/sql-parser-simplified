/*
Parser parses tokens into AST
 */

use crate::lib::ast::statements::Statement;
use crate::lib::dialect::Dialect;
use crate::lib::lexing::tokenizer::{Tokenizer, TokenizerError};
use crate::lib::lexing::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    /// Index of the first unprocessed token in `self.tokens`
    index: usize,
}

impl Parser {
    pub fn new(dialect: &dyn Dialect) -> Self {
        Self {
            tokens: vec![],
            index: 0,
        }
    }
    pub fn new_with_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }
    pub fn parse_statement(&mut self, sqlstr: &str) -> Result<Statement, ParserError> {
        let tokens = Tokenizer::new(sqlstr).tokenize()?;
        self.tokens = tokens;
        self.index = 0;
        Ok(Statement::Insert {
            table_name: "table".to_string(),
            columns: vec![],
            source: None,
        })
    }

    /// Return the first non-whitespace token that has not been processed
    /// and mark it as processed.
    pub fn next_token(&mut self) -> Token {
        loop {
            self.index += 1;
            match self.tokens.get(self.index - 1) {
                Some(Token::Whitespace(_)) => continue,
                Some(token) => return token.clone(),
                None => return Token::EOF,
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    TokenizerError(TokenizerError),
    ParserError(String),
    RecursionLimitExceeded,
}

impl From<TokenizerError> for ParserError {
    fn from(error: TokenizerError) -> Self {
        ParserError::TokenizerError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lib::lexing::tokens::{Whitespace, Word};

    #[test]
    fn test_next_token() {
        let mut parser = Parser::new_with_tokens(vec![
            Token::Word(Word {
                value: "SELECT".to_string(),
            }),
            Token::Whitespace(Whitespace::Space),
            Token::Number("1".to_string()),
            Token::EOF,
        ]);

        assert_eq!(
            parser.next_token(),
            Token::Word(Word {
                value: "SELECT".to_string()
            })
        );
        assert_eq!(parser.next_token(), Token::Number("1".to_string()));
        assert_eq!(parser.next_token(), Token::EOF);
    }
}
