use std::fmt::Debug;

/// A SQL dialect (PostgresSQL, MySQL, Spark, Ducksdb, etc.).
/// SQL implementations differ between engines.
pub trait Dialect: Debug {
    /// Identifiers are names used for database objects like tables, columns, etc.
    /// Different SQL dialects have different rules for what characters can start an identifier (e.g., letters, underscore).
    fn is_identifier_start(&self, ch: char) -> bool;
}
