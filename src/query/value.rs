use std::fmt::Display;

use super::{condition::Case, select::Select};




pub enum Value<'a> {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Str(&'a str),
    Bool(bool),
    Raw(String),
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::U32(v) => write!(f, "{}", v),
            Value::U64(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
            Value::Raw(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "'{}'", v),
            Value::Str(v) => write!(f, "'{}'", v),
            Value::Bool(v) => match v {
                true => write!(f, "TRUE"),
                false => write!(f, "FALSE"),
            }
        }
    }
}


impl<'a> Into<Value<'a>> for &'a str {
    fn into(self) -> Value<'a> {
        Value::Str(self)
    }
}
impl<'a> Into<Value<'a>> for String {
    fn into(self) -> Value<'a> {
        Value::String(self)
    }
}
impl<'a> Into<Value<'a>> for i32 {
    fn into(self) -> Value<'a> {
        Value::I32(self)
    }
}
impl<'a> Into<Value<'a>> for i64 {
    fn into(self) -> Value<'a> {
        Value::I64(self)
    }
}
impl<'a> Into<Value<'a>> for f32 {
    fn into(self) -> Value<'a> {
        Value::F32(self)
    }
}
impl<'a> Into<Value<'a>> for f64 {
    fn into(self) -> Value<'a> {
        Value::F64(self)
    }
}
impl<'a> Into<Value<'a>> for bool {
    fn into(self) -> Value<'a> {
        Value::Bool(self)
    }
}
impl<'a> Into<Value<'a>> for Case<'a> {
    fn into(self) -> Value<'a> {
        Value::Raw(format!("{}", self))
    }
}

impl<'a> Into<Value<'a>> for Select<'a> {
    fn into(self) -> Value<'a> {
        Value::Raw(format!("(\n {} \n)", self))
    }
}