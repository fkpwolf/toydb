use super::super::engine::Transaction;
use super::super::types::{Column, Expression};
use super::{Executor, ResultSet};
use crate::error::Result;

/// A table scan executor
pub struct Scan {
    table: String,
    filter: Option<Expression>,
}

impl Scan {
    pub fn new(table: String, filter: Option<Expression>) -> Box<Self> {
        Box::new(Self { table, filter })
    }
}

impl<T: Transaction> Executor<T> for Scan {
    fn execute(self: Box<Self>, txn: &mut T) -> Result<ResultSet> {
        let table = txn.must_read_table(&self.table)?;
        Ok(ResultSet::Query {
            columns: table.columns.iter().map(|c| Column { name: Some(c.name.clone()) }).collect(),
            rows: Box::new(txn.scan(&table.name, self.filter)?),
        })
    }
}
