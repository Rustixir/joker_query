

pub enum Func<'a> {
    Count(&'a str),
    Sum(&'a str),
    Avg(&'a str),
    Min(&'a str),
    Max(&'a str),
}
impl<'a> Func<'a> {
    pub fn count(v: &'a str) -> String {
        format!("COUNT({})", v)
    }
    pub fn sum(v: &'a str) -> String {
        format!("SUM({})", v)
    }
    pub fn avg(v: &'a str) -> String {
        format!("AVG({})", v)
    }
    pub fn min(v: &'a str) -> String {
        format!("MIN({})", v)
    }
    pub fn max(v: &'a str) -> String {
        format!("MAX({})", v)
    }
}


impl<'a> ToString for Func<'a> {
    fn to_string(&self) -> String {
        match self {
            Func::Count(v) => format!("COUNT({})", v),
            Func::Sum(v) => format!("SUM({})", v),
            Func::Avg(v) => format!("AVG({})", v),
            Func::Min(v) => format!("MIN({})", v),
            Func::Max(v) => format!("MAX({})", v),
        }
    }
}


