use std::fmt::Display;

use super::{value::Value, where_by::{WhereInfo, CondSep}, where_cond::{WhereCond, WhereOwner}, operator::Op, set::Pair};


pub struct Update<'a> {
    table: &'a str,
    kv: Vec<(&'a str, Value<'a>)>,
    whereas: Vec<WhereInfo<'a>>,
}


impl<'a> WhereOwner<'a> for Update<'a> {
    fn set_seperator(&mut self, index: usize, sep: CondSep) {
        self.whereas[index].seperator = Some(sep);
    }

    fn push(&mut self, where_info: WhereInfo<'a>) {
        self.whereas.push(where_info);
    }

    fn len(&self) -> usize {
        self.whereas.len()
    }
}


impl<'a> Display for Update<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "UPDATE {}\nSET ", self.table);
        let len = self.kv.len() - 1;
        for (index, (field, value)) in self.kv.iter().enumerate() {
            let _ = write!(f, "{} = {}", field, value);
            if index < len {
                let _ = write!(f, ", ");
            }
        }
        if self.whereas.len() > 0 {
            let _ = write!(f, "\nWHERE");
            self.whereas.iter().for_each(|c| {
                let _ = write!(f, "{}", c);
            });
        }
        return Ok(())
    }
}

impl<'a> Update<'a> {    
    pub fn table(table: &'a str) -> Update {
        Update { 
            table, 
            kv: vec![],
            whereas: vec![]
        }
    }

    pub fn set(mut self, field_value: Vec<Pair<'a>>) -> Self {
        for pair in field_value {
           self.kv.push((pair.field, pair.value)) 
        }
        return self
    }

    pub fn where_by(self, left: &'a str, op: Op<'a>) -> WhereCond<Self> {
        WhereCond::new(self, left, op)
    }

    pub fn build(&self) -> String {
        format!("{}", self)
    }
}

