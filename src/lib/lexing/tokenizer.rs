/*
Tokenizer (Lexer) parses SQLString into SQL tokens.
 */

use crate::lib::lexing::tokens::{Token, Whitespace, Word};
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    query: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut buf: Vec<TokenWithLocation> = vec![];
        let mut state = State {
            peekable: self.query.chars().peekable(),
            line: 1,
            col: 1,
        };
        let mut location = state.location();

        while let Some(token) = self.next_token(&mut state)? {
            buf.push(TokenWithLocation { token, location });
            location = state.location();
        }

        // into_iter() method is used here instead of iter() when we want to transform a collection into
        // another collection by owning its element (token here moved into new collection).
        // iter() returns an iterator over the items of a collection by reference,
        // meaning it allows you to read the items without changing the original collection which is not enough here.
        // into_iter() is used because we want to transform twl (a Vec<TokenWithLocation>) into a new Vec<Token>
        // by extracting the token field from each TokenWithLocation. This operation consumes twl.
        let tokens = buf.into_iter().map(|t| t.token).collect();
        Ok(tokens)
    }

    /// Get next token or return None
    fn next_token(&self, chars: &mut State) -> Result<Option<Token>, TokenizerError> {
        match chars.peek() {
            None => Ok(None),
            Some(&ch) => match ch {
                ' ' => {
                    chars.next();
                    Ok(Some(Token::Whitespace(Whitespace::Space)))
                }
                // all characters from '0' to '9' inclusive
                '0'..='9' => {
                    chars.next();
                    Ok(Some(Token::Number(ch.to_string())))
                }
                // string chars. TODO should check only valid keyword like SELECT, INSERT instead of all
                ch if ch.is_ascii() => self.tokenize_identifier_or_keyword([ch], chars),

                other => Err(TokenizerError {
                    message: format!("Unexpected character: {}", other),
                    location: chars.location(),
                }),
            },
        }
    }

    fn tokenize_identifier_or_keyword(
        &self,
        ch: impl IntoIterator<Item = char>,
        chars: &mut State,
    ) -> Result<Option<Token>, TokenizerError> {
        let ch: String = ch.into_iter().collect();
        let word: String = self.tokenize_word(ch, chars);

        Ok(Some(Token::Word(Word { value: word })))
    }

    fn tokenize_word(&self, first_chars: impl Into<String>, chars: &mut State) -> String {
        chars.next(); // consumer the first char
        let mut s: String = first_chars.into();
        s.push_str(&peeking_take_while(chars, |ch| {
            // TODO have a dialect here to check whether a char should be included in the word.
            ch.is_alphabetic()
        }));
        s
    }
}

fn peeking_take_while(chars: &mut State, mut predicate: impl FnMut(char) -> bool) -> String {
    let mut s: String = String::new();
    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            chars.next();
            s.push(ch);
        } else {
            break;
        }
    }
    s
}

#[derive(Debug)]
pub struct TokenizerError {
    pub message: String,
    pub location: Location,
}

/// Location in input string
#[derive(Debug)]
pub struct Location {
    // line starting from 1
    pub line: u64,
    // column starting from 1
    pub column: u64,
}

#[derive(Debug)]
pub struct TokenWithLocation {
    pub token: Token,
    pub location: Location,
}

/// An iterator of chars that can be peeked into with current line, col that the cursor is currently at.
struct State<'a> {
    peekable: Peekable<Chars<'a>>,
    pub line: u64,
    pub col: u64,
}

impl<'a> State<'a> {
    /// return next char and advance stream
    pub fn next(&mut self) -> Option<char> {
        match self.peekable.next() {
            None => None,
            Some(character) => {
                if character == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(character)
            }
        }
    }

    /// return the next character but do not advance the stream
    pub fn peek(&mut self) -> Option<&char> {
        self.peekable.peek()
    }

    pub fn location(&self) -> Location {
        Location {
            line: self.line,
            column: self.col,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::lexing::tokenizer::Tokenizer;
    use crate::lib::lexing::tokens::{Token, Whitespace, Word};

    #[test]
    fn tokenize_simple_select() {
        let sql = "SELECT 1";
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        println!("{tokens:?}");

        let expected = vec![
            Token::Word(Word {
                value: "SELECT".to_string(),
            }),
            Token::Whitespace(Whitespace::Space),
            Token::Number("1".to_string()),
        ];
        assert_eq!(tokens, expected)
    }

    #[test]
    fn tokenize_number() {
        let sql = " 1";
        let tokens = Tokenizer::new(sql).tokenize().unwrap();

        let expected = vec![
            Token::Whitespace(Whitespace::Space),
            Token::Number("1".to_string()),
        ];
        assert_eq!(tokens, expected)
    }

    #[test]
    fn tokenize_select_from() {
        let sql = "SELECT col FROM table";
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        println!("{tokens:?}");

        let expected = vec![
            Token::Word(Word {
                value: "SELECT".to_string(),
            }),
            Token::Whitespace(Whitespace::Space),
            Token::Word(Word {
                value: "col".to_string(),
            }),
            Token::Whitespace(Whitespace::Space),
            Token::Word(Word {
                value: "FROM".to_string(),
            }),
            Token::Whitespace(Whitespace::Space),
            Token::Word(Word {
                value: "table".to_string(),
            }),
        ];
        assert_eq!(tokens, expected)
    }
}
