
#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    /// The value of the identifier without quotes.
    pub value: String,
    /// The starting quote if any. Valid quote characters are the single quote,
    /// double quote, backtick, and opening square bracket.
    pub quote_style: Option<char>,
}

impl Ident {
    pub fn new<S>(value: S) -> Self where S: Into<String> {
        Ident {
            value: value.into(),
            quote_style: None,
        }
    }
}