use super::select::Select;
use std::fmt::Display;

pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
}

pub struct JoinInfo<'a> {
    typ: JoinType,
    join_table: &'a str,
    join_table_col: &'a str,
    table_col: &'a str,
}

impl<'a> Display for JoinInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self.typ {
            JoinType::Inner => write!(f, "\nINNER JOIN"),
            JoinType::Left => write!(f, "\nLEFT JOIN"),
            JoinType::Right => write!(f, "\nRIGHT JOIN"),
            JoinType::Outer => write!(f, "\nOUTER JOIN"),
        };
        write!(
            f,
            " {} ON {} = {}.{}",
            self.join_table, self.table_col, self.join_table, self.join_table_col
        )
    }
}

pub struct Join<'a> {
    selector: Select<'a>,
    info: JoinInfo<'a>
}
impl<'a> Join<'a> {
    pub fn from(selector: Select<'a>, join_table: &'a str, typ: JoinType) -> Self {
        Join {
            selector,
            info: JoinInfo { 
                typ, 
                join_table,
                join_table_col: "",
                table_col: "", 
            }
        }
    }

    pub fn on(mut self, table_col: &'a str, join_table_col: &'a str) -> Select<'a> {
        self.info.table_col = table_col;
        self.info.join_table_col = join_table_col;
        self.selector.joins.push(self.info);
        self.selector
    }
}
