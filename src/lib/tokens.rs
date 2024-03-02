/*

Tokens are the basic syntactical/ lexical units of SQL.
A token is a sequence of one or more characters.
A token cannot contain blank characters, unless it is a string constant or a delimited identifier, which may contain blanks.
Delimiters are used to separate tokens.

Types of tokens
- identifiers
- constants
- keywords
 */

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Word(Word),
    Number(String),

    Comma,
    Whitespace(Whitespace),

    // operators start
    /// Equality operator `=`
    Eq,
    Neq,
    Lt,
    Gt,

    Plus,
    Minus,
    Mul,
    Div,

    LParen,
    RParen,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Whitespace {
    Space,
    Newline,
    Tab,
}

/// Word has String which is heap-allocated to it is Clone instead of Copy.
/// Copy is mostly for primitive types which is copied bitwise.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Word {
    pub value: String,
}
