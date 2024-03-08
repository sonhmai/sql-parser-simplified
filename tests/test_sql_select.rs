use sqlparsing::lib::ast::statements::Statement;
use sqlparsing::lib::dialect::GenericDialect;
use sqlparsing::lib::parsing::Parser;

#[test]
fn parse_simple_select() {
    let sql = "SELECT id, first_name, last_name FROM customer";
    let dialect = GenericDialect {};
    let mut parser = Parser::new(&dialect);
    let ast: Statement = parser.parse_statement(sql).unwrap();

    println!("{:?}", ast);
    // ensure that sql string parses as a single Query
    // assert_eq!(ast, Statement::Query)
}
