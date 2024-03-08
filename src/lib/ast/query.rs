use crate::lib::ast::set_expr::SetExpr;

/// select
#[derive(Debug)]
pub struct Query {
    pub body: Box<SetExpr>,
}
