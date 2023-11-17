// use super::value::{Value, self};

use std::string::FromUtf8Error;

use super::value::{self, Value};

#[derive(Debug)]
pub enum Error {
    Utf8Error(FromUtf8Error),
}



pub struct Statement {
    pub query: String,
}

impl Statement {
    pub fn bind<'a>(&self, params: Vec<Value<'a>>) -> Result<String, Error> {
        let mut params_start = 0;
        let params_end = params.len();

        // create Vec with at-least self.query size
        let mut query = Vec::<u8>::with_capacity(self.query.len());
        let bytes = self.query.as_bytes();
        let mut cursor = 0;
        let end = bytes.len();
        let mut finish = false;
        while cursor < end {
            if !finish && bytes[cursor] == b'{' && bytes[cursor + 1] == b'}' {
                let param = format!("{}", params[params_start]);
                query.extend_from_slice(param.as_bytes());
                params_start += 1;
                cursor += 2;
                if params_start == params_end {
                    finish = true;
                }
                continue;
            }
            query.push(bytes[cursor]);
            cursor += 1;
        }

        let q = String::from_utf8(query);
        match q {
            Ok(v) => Ok(v),
            Err(e) => return Err(Error::Utf8Error(e)),
        }
    }
}

impl<'a> From<String> for Statement {
    fn from(query: String) -> Self {
        let query = query.replace(value::PARAM_STR, "{}");
        let query = query.replace(value::PARAM, "{}");

        Statement {
            query,
        }
    }
}
