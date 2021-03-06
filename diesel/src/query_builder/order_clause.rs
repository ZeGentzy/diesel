simple_clause!(NoOrderClause, OrderClause, " ORDER BY ");

impl<'a, DB, Expr> Into<Option<Box<QueryFragment<DB> + 'a>>> for OrderClause<Expr>
where
    DB: Backend,
    Expr: QueryFragment<DB> + 'a,
{
    fn into(self) -> Option<Box<QueryFragment<DB> + 'a>> {
        Some(Box::new(self.0))
    }
}

impl<'a, DB> Into<Option<Box<QueryFragment<DB> + 'a>>> for NoOrderClause
where
    DB: Backend,
{
    fn into(self) -> Option<Box<QueryFragment<DB> + 'a>> {
        None
    }
}
