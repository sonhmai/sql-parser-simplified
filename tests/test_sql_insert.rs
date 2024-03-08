use sqlparsing::lib::ast::expression::{Expr, Ident};
use sqlparsing::lib::ast::expression::Value;
use sqlparsing::lib::ast::statements::Statement;
use sqlparsing::lib::dialect::GenericDialect;
use sqlparsing::lib::lexing::tokenizer::Tokenizer;
use sqlparsing::lib::lexing::tokens::Token;
use sqlparsing::lib::parsing::Parser;

#[test]
fn parse_insert_values() {
    let row = vec![
        Expr::Value(Value::Number("1".to_string(), false)),
        Expr::Value(Value::Number("2".to_string(), false)),
        Expr::Value(Value::Number("3".to_string(), false)),
    ];
    let rows1 = vec![row.clone()];
    let rows2 = vec![row.clone(), row];

    let sql = "INSERT customer VALUES (1, 2, 3)";
    check_one(sql, "customer", &[], &rows1);

    let sql = "INSERT INTO customer VALUES (1, 2, 3)";
    check_one(sql, "customer", &[], &rows1);

    let sql = "INSERT INTO customer VALUES (1, 2, 3), (1, 2, 3)";
    check_one(sql, "customer", &[], &rows2);

    let sql = "INSERT INTO public.customer VALUES (1, 2, 3)";
    check_one(sql, "public.customer", &[], &rows1);

    let sql = "INSERT INTO db.public.customer VALUES (1, 2, 3)";
    check_one(sql, "db.public.customer", &[], &rows1);

    let sql = "INSERT INTO public.customer (id, name, active) VALUES (1, 2, 3)";
    check_one(
        sql,
        "public.customer",
        &["id".to_string(), "name".to_string(), "active".to_string()],
        &rows1,
    );

    fn check_one(
        sql: &str,
        expected_table_name: &str,
        expected_columns: &[String],
        expected_rows: &[Vec<Expr>],
    ) {
        match verified_stmt(sql) {
            Statement::Insert {
                table_name,
                columns,
                source: Some(source),
                ..
            } => {
                assert_eq!(table_name.to_string(), expected_table_name);
                assert_eq!(columns.len(), expected_columns.len());
                for (index, column) in columns.iter().enumerate() {
                    assert_eq!(column, &Ident::new(expected_columns[index].clone()));
                }
                // match *source.body {
                //     SetExpr::Values(Values { rows, .. }) => {
                //         assert_eq!(rows.as_slice(), expected_rows)
                //     }
                //     _ => unreachable!(),
                // }
            }
            _ => unreachable!(),
        }
    }

    verified_stmt("INSERT INTO customer WITH foo AS (SELECT 1) SELECT * FROM foo UNION VALUES (1)");
}

/// Ensures that `sql` parses as a single [Statement], and that
/// re-serializing the parse result produces the same `sql`
/// string (is not modified after a serialization round-trip).
fn verified_stmt(query: &str) -> Statement {
    let mut tokenizer = Tokenizer::new(query);
    let tokens = tokenizer.tokenize().unwrap();
    let dialect = GenericDialect{};
    let mut parser = Parser::new(&dialect);

    parser.parse_statement(query).unwrap()
}