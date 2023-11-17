use std::fmt::Display;

use super::{value::Value, expression::Exp};


// Case is sql Case maker
pub struct Case<'a> {
    whens: Vec<(Exp<'a>, Value<'a>)>,
    else_value: Option<Value<'a>>
}

impl<'a> Case<'a> {
    pub fn new() -> Self {
        Case { 
            whens: vec![],
            else_value: None
        }
    }

    pub fn when(self, exp: Exp<'a>) -> When<'a> {
        When { cases: self, exp }
    }

    pub fn else_value(mut self, value: Value<'a>) -> Case<'a> {
        self.else_value = Some(value);
        self
    }
}

impl<'a> Display for Case<'a>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "CASE\n");
        self.whens.iter().for_each(|(e, v)| {
            let _ = write!(f, "WHEN {} THEN {}\n", e, v);
        });
        if let Some(v) = &self.else_value {
            let _ = write!(f, "ELSE {}\n", v);
        }
        let _ = write!(f, "END\n");
        Ok(())
    }
}


pub struct When<'a> {
    cases: Case<'a>,
    exp: Exp<'a>
}

impl<'a> When<'a> {
    pub fn then(mut self, value: Value<'a>) -> Case<'a> {
        self.cases.whens.push((self.exp, value));
        self.cases
    }
}