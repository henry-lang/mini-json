use std::fmt;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::{Json, Map};

pub struct Parser<'a> {
    iter: Peekable<Enumerate<Chars<'a>>>,
}

pub enum ParseJsonError {
    Expected(char, char),
    EndOfString(char),
}

impl fmt::Display for ParseJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected(expected, found) => {
                write!(f, "Expected {}, found {}.", expected, found)
            }
            Self::EndOfString(expected) => {
                write!(f, "Expected {}, found end of string.", expected)
            }
        }
    }
}

type JsonResult = Result<Json, ParseJsonError>;

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            iter: source.chars().enumerate().peekable(),
        }
    }

    fn consume(&mut self, expected: char) -> Result<(), ParseJsonError> {
        if self.iter.peek().is_none() {
            return Err(ParseJsonError::EndOfString(expected));
        }

        let next = self.iter.next_if(|(_, next)| *next == expected);
        match next {
            Some(_) => Ok(()),
            None => Err(ParseJsonError::Expected(
                expected,
                self.iter.peek().unwrap().1,
            )),
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .iter
            .next_if(|(_, next)| next.is_whitespace())
            .is_some()
        {}
    }

    fn parse_string(&mut self) -> JsonResult {
        self.iter.next(); // Opening quote
        while self.iter.next_if(|(_, next)| *next != '"').is_some() {}
        self.consume('"')?;

        Ok(Json::String("todo".into()))
    }

    pub fn parse_object(&mut self) -> JsonResult {
        let mut data = Map::new();

        self.skip_whitespace();
        self.consume('{')?;
        self.skip_whitespace();

        while self.iter.peek().is_some() && self.iter.peek().unwrap().1 == '"' {
            let key = match self.parse_string()? {
                Json::String(key) => key,
                _ => unreachable!(),
            };
            self.consume(':')?;
            let value = self.parse_string()?; // Temporary, will be parse_object later.

            data.insert(key, value);
        }

        Ok(Json::Object(data))
    }
}
