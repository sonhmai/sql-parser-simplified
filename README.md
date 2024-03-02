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

SQLString --Lexer/Tokenizer--> Tokens --Parser--> AbstractSyntaxTree

Expression
- every node in AST is an expression
- there is only a single set of Expression types for all dialects.


Refs
- [Primer on SQLGlot's Abstract Syntax Tree](https://github.com/tobymao/sqlglot/blob/main/posts/ast_primer.md)
- [Rust crate sqlparser](https://docs.rs/sqlparser/0.43.1/sqlparser/): used in Apache `arrow-datafusion` and many other places in Rust ecosystem.
