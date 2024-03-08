#[derive(Debug, Clone)]
pub enum Value {
    Number(String, bool),
    SingleQuotedString(String),
    Boolean(bool),
}
