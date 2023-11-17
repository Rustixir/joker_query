use std::fmt::Display;



pub enum OrderType  {
    ASC,
    DESC
}
impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::ASC => write!(f, "ASC"),
            OrderType::DESC => write!(f, "DESC"),
        }
    }
}


pub struct Order<'a> {
    col: &'a str,
    typ: OrderType
}
impl<'a> Order<'a> {
    pub fn new(col: &'a str, typ: OrderType)  -> Self {
        Order { col, typ }
    }
}

impl<'a> Display for Order<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.col, self.typ)
    }
}