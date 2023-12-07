use std::fmt::Display;

use super::{where_by::{WhereInfo, CondSep}, operator::Op, expression::Exp};


pub trait WhereOwner<'a> {
    fn set_seperator(&mut self, index: usize, sep: CondSep);
    fn push(&mut self, where_info: WhereInfo<'a>);
    fn len(&self) -> usize;
}



pub struct WhereCond<T> {
    owner: T
    
}

impl<'a, T> WhereCond<T>  
where 
    T: WhereOwner<'a> + Display
{
    
    pub fn new(owner: T, left: &'a str, op: Op<'a>) -> Self {
        let mut tmp = WhereCond { owner };
        tmp.owner.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        tmp
    }

    pub fn and(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.owner.len();
        self.owner.set_seperator(len - 1, CondSep::And);
        self.owner.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }
    
    pub fn or(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.owner.len();
        self.owner.set_seperator(len - 1, CondSep::Or);
        self.owner.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }

    pub fn not(mut self, left: &'a str, op: Op<'a>) -> Self {
        let len = self.owner.len();
        self.owner.set_seperator(len - 1, CondSep::Not);
        self.owner.push(WhereInfo { 
            exp: Exp::new(left, op), 
            seperator: None 
        });
        return self;
    }



    pub fn build(self) -> String {
        format!("{}", self.owner)
    }

}