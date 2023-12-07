use std::fmt::Display;

use super::{array::Array, select::Select};



pub struct Insert<'a> {
    table: &'a str,
    columns: Vec<&'a str>,
    values: Vec<Array<'a>>,
    source: Option<Select<'a>>,
}


impl<'a> Display for Insert<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "INSERT INTO {} (", self.table);
        
        let col_len = self.columns.len() - 1;
        for (index, col) in self.columns.iter().enumerate() {
            let _ = write!(f, "{}",col);
            if index < col_len {
                let _ = write!(f, ", ");
            }
        }
        let _ = write!(f, ")\n");

        if let Some(selector) = &self.source {
            let _ = write!(f, "{}", selector.build());
            return Ok(())
        }

        let _ = write!(f, "VALUES\n");

        let records_len = self.values.len() - 1;
        for (index, record) in self.values.iter().enumerate() {
            let _ = write!(f, "(");
            let record_len = record.array.len() - 1;
            for (i, val) in record.array.iter().enumerate() {
                let _ = write!(f, "{}", val);
                if i < record_len {
                    let _ = write!(f, ", ");    
                }
            }
            let _ = write!(f, ")");
            if index < records_len {
                let _ = write!(f, ", \n");
            }
        }

        Ok(())
    }
}


impl<'a> Insert<'a> {
    pub fn into(table: &'a str) -> Self {
        Insert { 
            table, 
            columns: vec![],
            values: vec![],
            source: None
        }
    }
    pub fn cols(mut self, cols: Vec<&'a str>) -> Self {
        self.columns = cols;
        return self
    }
    pub fn value(mut self, record: Array<'a>) -> Self {
        self.values.push(record);
        return self
    }
    pub fn values(mut self, records: Vec<Array<'a>>) -> Self {
        for rec in records {
            self.values.push(rec);
        }
        return self
    }
    pub fn source(mut self, selector: Select<'a>) -> Self {
        self.source = Some(selector);
        return self
    }

    pub fn build(&self) -> String {
        format!("{}", self)
    }
}
