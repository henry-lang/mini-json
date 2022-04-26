use std::collections::HashMap;
use std::str::FromStr;

mod parser;

pub use parser::{ParseJsonError, Parser};

type Map<K, V> = HashMap<K, V>;

pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Map<String, Json>),
}

impl FromStr for Json {
    type Err = ParseJsonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse_object()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let json = r#"{"Paris": "France"}"#.parse::<Json>().unwrap();
        let values = match json {
            Json::Object(values) => values,
            _ => unreachable!(),
        };
        assert!(values.get("todo").is_some());
    }
}
