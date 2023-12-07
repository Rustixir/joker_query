use std::fmt::Display;

use super::{where_cond::{WhereOwner, WhereCond}, where_by::WhereInfo, operator::Op};


pub struct Delete<'a> {
    table: &'a str,
    whereas: Vec<WhereInfo<'a>>,
}

impl<'a> WhereOwner<'a> for Delete<'a> {
    fn set_seperator(&mut self, index: usize, sep: super::where_by::CondSep) {
        self.whereas[index].seperator = Some(sep);
    }

    fn push(&mut self, where_info: super::where_by::WhereInfo<'a>) {
        self.whereas.push(where_info);
    }

    fn len(&self) -> usize {
        self.whereas.len()
    }
}

impl<'a> Display for Delete<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "DELETE FROM {}", self.table);
        if self.whereas.len() > 0 {
            let _ = write!(f, "\nWHERE");
            self.whereas.iter().for_each(|c| {
                let _ = write!(f, "{}", c);
            });
        }
        return Ok(())
    }
}


impl<'a> Delete<'a> {

    pub fn from(table: &'a str) -> Self {
        Delete { table, whereas: vec![] }
    }

    pub fn where_by(self, left: &'a str, op: Op<'a>) -> WhereCond<Self> {
        WhereCond::new(self, left, op)
    }

    pub fn build(&self) -> String {
        format!("{}", self)
    }
    
}