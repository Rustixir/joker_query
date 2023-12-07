use std::fmt::Display;

use super::{select::Select, operator::Op, expression::Exp, where_cond::WhereOwner};


pub enum CondSep {
    And,
    Or,
    Not
}

pub struct WhereInfo<'a> {
    pub exp: Exp<'a>,
    pub seperator: Option<CondSep>
}
impl<'a> Display for WhereInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, " {}", self.exp);
        match &self.seperator {
            None => {
                return Ok(())
            }
            Some(c) => {
                let _ = match c {
                    CondSep::And => write!(f, "\nAND"),
                    CondSep::Or  => write!(f, "\nOR"),
                    CondSep::Not => write!(f, "\nNOT"),
                };
            }
        }
        return Ok(());
    }
}


pub struct Where<'a> {
    selector: Select<'a>,
    
}

impl<'a> Into<Select<'a>> for Where<'a> {
    fn into(self) -> Select<'a> {
        self.selector
    }
}

impl<'a> Where<'a> {
    
    pub fn new(selector: Select<'a>, left: &'a str, op: Op<'a>) -> Self {
        let mut tmp = Where { selector };
        tmp.selector.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        tmp
    }

    pub fn and(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.selector.len();
        self.selector.set_seperator(len - 1, CondSep::And);
        self.selector.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }
    
    pub fn or(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.selector.len();
        self.selector.set_seperator(len - 1, CondSep::Or);
        self.selector.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }

    pub fn not(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.selector.len();
        self.selector.set_seperator(len - 1, CondSep::Not);
        self.selector.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }


    pub fn group_by(mut self, cols: Vec<&'a str>) -> Select<'a> {
        self.selector.groups = cols;
        return self.selector
    }


    pub fn order_by(self, col: &'a str) -> Select<'a> {
        return self.selector.order_by(col)
    }
    pub fn order_by_desc(self, col: &'a str) -> Select<'a> {
        return self.selector.order_by_desc(col)
    }
    

    pub fn limit(mut self, num: u32) -> Select<'a> {
        self.selector.limit = Some(num);
        return self.selector
    }

    pub fn offset(mut self, num: u32) -> Select<'a> {
        self.selector.offset = Some(num);
        return self.selector
    }
    

    pub fn build(self) -> String {
        format!("{}", self.selector)
    }

}