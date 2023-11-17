use std::fmt::Display;

use dashmap::{DashMap, mapref::one::Ref};

use super::{
    having::{Having, HavingInfo},
    join::{Join, JoinInfo, JoinType},
    operator::Op,
    order_by::{Order, OrderType},
    where_by::{Where, WhereInfo}, statement::{Statement, self}, value::Value,
};




lazy_static! {
    static ref SELECT_CACHE: DashMap<String, Statement> = DashMap::new();
}


pub struct Stmt<'a> {
    stmt: Ref<'a, String, Statement>
}

impl<'a> Stmt<'a> {
    pub fn bind(self, params: Vec<Value<'a>>) -> Result<String, statement::Error> {
        self.stmt.value().bind(params)
    }
}


pub struct Select<'a> {
    pub table: &'a str,
    pub distinct: Option<()>,
    pub cols: Vec<&'a str>,
    pub joins: Vec<JoinInfo<'a>>,
    pub whereas: Vec<WhereInfo<'a>>,
    pub groups: Vec<&'a str>,
    pub havings: Vec<HavingInfo<'a>>,
    pub orders: Vec<Order<'a>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}


impl<'a> Display for Select<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "SELECT ");
        if self.cols.len() == 0 {
            let _ = write!(f, "*");
        } else {
            if let Some(_) = self.distinct {
                let _ = write!(f, "DISTINCT ");
            }
        }
        self.cols.iter().enumerate().for_each(|(index, c)| {
            let _ = write!(f, "{}", c);
            if index < (self.cols.len() - 1) {
                let _ = write!(f, ", ");
            }
        });
        let _ = write!(f, " FROM {}", self.table);

        self.joins.iter().for_each(|c| {
            let _ = write!(f, "{}", c);
        });

        if self.whereas.len() > 0 {
            let _ = write!(f, "\nWHERE");
            self.whereas.iter().for_each(|c| {
                let _ = write!(f, "{}", c);
            });
        }

        if self.groups.len() > 0 {
            let _ = write!(f, "\nGROUP BY");
            self.groups.iter().enumerate().for_each(|(index, c)| {
                let _ = write!(f, " {}", c);
                if index < (self.groups.len() - 1) {
                    let _ = write!(f, ", ");
                }
            });
        }

        if self.havings.len() > 0 {
            let _ = write!(f, "\nHAVING");
            self.havings.iter().for_each(|c| {
                let _ = write!(f, "{}", c);
            });
        }

        if self.orders.len() > 0 {
            let _ = write!(f, "\nORDER BY ");
            self.orders.iter().enumerate().for_each(|(i, c)| {
                let _ = write!(f, "{}", c);
                if i < self.orders.len() - 1 {
                    let _ = write!(f, ", ");
                }
            });
        }

        if let Some(v) = self.limit {
            let _ = write!(f, "\nLIMIT {}", v);
        }

        if let Some(v) = self.offset {
            let _ = write!(f, "\nOFFSET {}", v);
        }

        Ok(())
    }
}

impl<'a> Select<'a> {
    fn new() -> Self {
        Select {
            distinct: None,
            cols: vec![],
            table: "",
            joins: vec![],
            whereas: vec![],
            groups: vec![],
            havings: vec![],
            orders: vec![],
            limit: None,
            offset: None,
        }
    }

    pub fn stmt(key: &String) -> Option<Stmt> {
        match SELECT_CACHE.get(key) {
            Some(stmt) => Some(Stmt { stmt }),
            None => None,
        }
    }

    pub fn any() -> Self {
        Select::new()
    }

    pub fn cols(cols: Vec<&'a str>) -> Self {
        let mut selector = Select::new();
        selector.cols = cols;
        selector
    }

    pub fn col(mut self, col: &'a str) -> Self {
        self.cols.push(col);
        return self;
    }

    pub fn distinct(mut self) -> Self {
        self.distinct = Some(());
        self
    }

    pub fn from(mut self, table: &'a str) -> Self {
        self.table = table;
        return self;
    }

    pub fn inner_join(self, table: &'a str) -> Join<'a> {
        Join::from(self, table, JoinType::Inner)
    }
    pub fn left_join(self, table: &'a str) -> Join<'a> {
        Join::from(self, table, JoinType::Left)
    }
    pub fn right_join(self, table: &'a str) -> Join<'a> {
        Join::from(self, table, JoinType::Right)
    }
    pub fn outer_join(self, table: &'a str) -> Join<'a> {
        Join::from(self, table, JoinType::Outer)
    }

    pub fn where_by(self, left: &'a str, op: Op<'a>) -> Where<'a> {
        Where::new(self, left, op)
    }

    pub fn group_by(mut self, cols: Vec<&'a str>) -> Self {
        self.groups = cols;
        return self;
    }

    pub fn order_by(mut self, col: &'a str) -> Self {
        self.orders.push(Order::new(col, OrderType::ASC));
        return self;
    }
    pub fn order_by_desc(mut self, col: &'a str) -> Self {
        self.orders.push(Order::new(col, OrderType::DESC));
        return self;
    }

    pub fn having(self, left: &'a str, op: Op<'a>) -> Having<'a> {
        Having::new(self, left, op)
    }

    

    pub fn limit(mut self, num: u32) -> Self {
        self.limit = Some(num);
        return self;
    }

    pub fn offset(mut self, num: u32) -> Self {
        self.offset = Some(num);
        return self;
    }

    pub fn build(&self) -> String {
        format!("{}", self)
    }

    pub fn prepare(&self) -> String {
        let query = self.build();    
        let statement = Statement::from(query);
        let key = uuid::Uuid::new_v4().to_string();
        let _ = SELECT_CACHE.insert(key.clone(), statement);
        return key
    }

}
