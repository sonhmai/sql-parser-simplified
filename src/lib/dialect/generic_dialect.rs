use crate::lib::dialect::Dialect;

#[derive(Debug)]
pub struct GenericDialect{}

impl Dialect for GenericDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic()
    }
}