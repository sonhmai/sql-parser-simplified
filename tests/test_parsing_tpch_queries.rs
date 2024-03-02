use sqlparsing::lib::ast::statements::Statement;
use sqlparsing::lib::dialect::GenericDialect;
use sqlparsing::lib::parsing::Parser;

/// QUERIES is a slice of string slices, each string slice is a TPCH SQL query.
///
/// Its type `&[&str]` is a slice of string slices
/// - &str: read-only pointer (borrowing without taking ownership) to a string slice
/// (seq of UTF-8 chars) of 1 TPCH query stored in memory.
/// - `&[T]` slice of T = contiguous seq of T
const QUERIES: &[&str] = &[
  include_str!("tpch/1.sql"),
  include_str!("tpch/2.sql"),
];

#[test]
fn test_tpch() {
  let dialect = GenericDialect{};
  let mut parser = Parser::new(&dialect);
  let ast: Statement = parser.parse_statement(QUERIES[0]).unwrap();

  println!("{}", QUERIES[0]);
  println!("{:?}", ast);
}