# SQL Parsing

Getting Started
``` 
cargo test
```


Structure
``` 
src // lib source code with unit tests
tests // integration test
```

SQLString --Lexer/Tokenizer--> Tokens --Parser--> Statement (AbstractSyntaxTree - AST)

SQL Statement (AST)
- A statement represents the level that is to be communicated with the relational database.
- AST is a data structure that represents a SQL statement (INSERT, SELECT, UPDATE, CREATE, BEGIN, etc.).


Expression
- every node in AST is an expression
- expression evaluates to value
- there is only a single set of Expression types for all dialects.

Identifier
- names of database objects (schema, table, column, etc.)

```sql
-- the whole str is a SQL statement (an AST)
SELECT --> SELECT expression
    col1, col2 --> column identifier
FROM --> From expression
    table1 --> Table identifier
```

Insert statement example
``` 
INSERT INTO
    customer
VALUES (1, 2)

-- tokenizer -> raw_tokens
Word(INSERT), Whitespace, Word(INFO), 
Whitespace, Word(customer), 
Whitespace, Word(VALUES),
Whitespace, LParen, Number(1), Comma, Whitespace, Number(2), RParen
EOF
-- removed whitespaces -> tokens
Word(INSERT), Word(INFO), Word(customer), 
Word(VALUES), LParen, Number(1), Comma, Number(2), RParen
EOF
-- parser -> Statement: INSERT
Statement:Insert
    into = true,
    table_name: ObjectName = customer,
    columns: Vec<Ident> = vec![],
    source: Option<Query> = Some(
        Query { 
            with: None, 
            body: Values(Values { 
                explicit_row: false, 
                rows: [
                    [Value(Number("1", false)), 
                    Value(Number("2", false)), 
                    Value(Number("3", false))]] 
        }
    )
```

Select statement example
``` 

```


Refs
- [Primer on SQLGlot's Abstract Syntax Tree](https://github.com/tobymao/sqlglot/blob/main/posts/ast_primer.md)
- [Rust crate sqlparser](https://docs.rs/sqlparser/0.43.1/sqlparser/): used in Apache `arrow-datafusion` and many other places in Rust ecosystem.
- [SQL Abstract Syntax Trees Vocabulary](http://ns.inria.fr/ast/sql/index.html)
- [Automatically detecting breaking changes in SQL queries](https://tobikodata.com/automatically-detecting-breaking-changes-in-sql-queries.html)
- [Build SQL parser using ANTLR4 - Part 1](https://medium.com/@sasidharc/build-sql-parser-using-antlr4-part1-2044916a8406)

