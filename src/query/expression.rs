use std::fmt::Display;
use super::operator::Op;


// Exp ~> Expression
pub struct Exp<'a> {
    left: &'a str,
    operator: Op<'a>,
}

impl<'a> Exp<'a> {
    pub fn new(left: &'a str, operator: Op<'a>) -> Self {
        Exp { left, operator }
    }
}


impl<'a> Display for Exp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.left, self.operator)
    }
}