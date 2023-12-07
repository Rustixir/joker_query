use std::fmt::Display;

use super::{array::Array, condition::Case, select::Select, value::Value};


pub fn is_null<'a>() -> Op<'a> {
    Op::IsNull
}
pub fn eq<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Eq(value.into())
}
pub fn neq<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Neq(value.into())
}
pub fn gt<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Gt(value.into())
}
pub fn gte<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Gte(value.into())
}
pub fn lt<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Lt(value.into())
}
pub fn lte<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Lte(value.into())
}
pub fn nlt<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Nlt(value.into())
}
pub fn ngt<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Ngt(value.into())
}
pub fn between<'a>(from: impl Into<Value<'a>>, to: impl Into<Value<'a>>) -> Op<'a> {
    Op::Between(from.into(), to.into())
}
pub fn not_between<'a>(from: impl Into<Value<'a>>, to: impl Into<Value<'a>>) -> Op<'a> {
    Op::NotBetween(from.into(), to.into())
}
pub fn exists<'a>(sub: Select<'a>) -> Op<'a> {
    Op::Exists(sub)
}
pub fn any<'a>(sub: Select<'a>) -> Op<'a> {
    Op::Any(sub)
}
pub fn in_array<'a>(value: impl Into<Array<'a>>) -> Op<'a> {
    Op::In(value.into())
}
pub fn not_in<'a>(value: impl Into<Array<'a>>) -> Op<'a> {
    Op::NotIn(value.into())
}
pub fn like<'a>(pattern: &'a str) -> Op<'a> {
    Op::Like(pattern)
}
pub fn not<'a>(value: impl Into<Value<'a>>) -> Op<'a> {
    Op::Not(value.into())
}
pub fn case<'a>(case: Case<'a>) -> Op<'a> {
    Op::Case(case)
}
pub fn none<'a>() -> Op<'a> {
    Op::None
}





pub enum Op<'a> {
    IsNull,
    // Eq ~> Equal
    Eq(Value<'a>),
    // Eq ~> Not Equal
    Neq(Value<'a>),
    // Gt ~> Greater Than
    Gt(Value<'a>),
    // Gte ~> Greater Than or Equal
    Gte(Value<'a>),
    // Lt ~> Less Than
    Lt(Value<'a>),
    // Lte ~> Less Than or Equal
    Lte(Value<'a>),
    // Nlt ~> Not Less Than
    Nlt(Value<'a>),
    // Ngt ~> Not Greater Than
    Ngt(Value<'a>),

    Between(Value<'a>, Value<'a>),

    NotBetween(Value<'a>, Value<'a>),

    Exists(Select<'a>),

    Any(Select<'a>),

    In(Array<'a>),

    NotIn(Array<'a>),

    Like(&'a str),

    Not(Value<'a>),

    Case(Case<'a>),

    None,
}    

impl<'a> Display for Op<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::IsNull => write!(f, "{}", IS_NULL),
            Op::Eq(v) => write!(f, "{} {}", EQ, v),
            Op::Neq(v) => write!(f, "{} {}", NEQ, v),
            Op::Gt(v) => write!(f, "{} {}", GT, v),
            Op::Gte(v) => write!(f, "{} {}", GTE, v),
            Op::Lt(v) => write!(f, "{} {}", LT, v),
            Op::Lte(v) => write!(f, "{} {}", LTE, v),
            Op::Nlt(v) => write!(f, "{} {}", NLT, v),
            Op::Ngt(v) => write!(f, "{} {}", NGT, v),
            Op::Between(v, v2) => write!(f, "{} {} {} {}", BETWEEN, v, AND, v2),
            Op::NotBetween(v, v2) => write!(f, "{} {} {} {}", NOT_BETWEEN, v, AND, v2),
            Op::Exists(v) => write!(f, "{} (\n {} \n)", EXISTS, v),
            Op::Any(v) => write!(f, "{} (\n {} \n)", ANY, v),
            Op::Like(v) => write!(f, "{} '{}'", LIKE, v),
            Op::Not(v) => write!(f, "{} {}", NOT, v),
            Op::Case(v) => write!(f, "{}", v),
            Op::None => Ok(()),
            Op::In(v) => {
                let _ = write!(f, "{} (", IN);
                v.array.iter().enumerate().for_each(|(index, elem)| {
                    let _ = elem.fmt(f);
                    if index < (v.array.len() - 1) {
                        let _ = write!(f, ", ");
                    }
                });
                return write!(f, ")");
            }
            Op::NotIn(v) => {
                let _ = write!(f, "{} (", NOT_IN);
                v.array.iter().enumerate().for_each(|(index, elem)| {
                    let _ = elem.fmt(f);
                    if index < (v.array.len() - 1) {
                        let _ = write!(f, ", ");
                    }
                });
                return write!(f, ")");
            }
        }
    }
}

type CompareOp = &'static str;
const EQ: CompareOp = "=";
const NEQ: CompareOp = "!=";
const GT: CompareOp = ">";
const GTE: CompareOp = ">=";
const LT: CompareOp = "<";
const LTE: CompareOp = "<=";
const NLT: CompareOp = "!<";
const NGT: CompareOp = "!>";

const BETWEEN: CompareOp = "BETWEEN";
const NOT_BETWEEN: CompareOp = "NOT BETWEEN";
const EXISTS: CompareOp = "EXISTS";
const ANY: CompareOp = "ANY";
const IN: CompareOp = "IN";
const NOT_IN: CompareOp = "NOT IN";
const LIKE: CompareOp = "LIKE";
const NOT: CompareOp = "NOT";
const IS_NULL: CompareOp = "IS NULL";
const AND: CompareOp = "AND";
