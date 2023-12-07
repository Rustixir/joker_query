use super::value::Value;


pub struct Pair<'a> {
    pub field: &'a str,
    pub value: Value<'a>
}

impl<'a> Pair<'a> {
    
    pub fn from(field: &'a str, value: impl Into<Value<'a>>) -> Pair<'a> {
        Pair { field, value:  value.into()}
    }
    
}