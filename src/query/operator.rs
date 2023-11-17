use std::fmt::Display;

use super::{select::Select, value::Value, array::Array};

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

    Exists(Select<'a>),

    In(Array<'a>),

    NotIn(Array<'a>),

    Like(&'a str),

    Not(Value<'a>),

    Case(Value<'a>),

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
            Op::Exists(v) => write!(f, "{} (\n {} \n)", EXISTS, v),
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
const EXISTS: CompareOp = "EXISTS";
const IN: CompareOp = "IN";
const NOT_IN: CompareOp = "NOT IN";
const LIKE: CompareOp = "LIKE";
const NOT: CompareOp = "NOT";
const IS_NULL: CompareOp = "IS NULL";
const AND: CompareOp = "AND";
