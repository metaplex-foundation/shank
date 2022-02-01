use std::ops::Deref;

use anyhow::{format_err, Error};
use proc_macro2::LineColumn;
use syn::Error as ParseError;

pub type ShankParseResult<T> = std::result::Result<T, ShankParseError>;

#[derive(Debug)]
pub struct ShankParseError {
    pub inner: ParseError,
    file: Option<String>,
}

const UNKNOWN_FILE: &str = "<unknown>";
impl ShankParseError {
    pub fn file(&self) -> String {
        self.file
            .as_ref()
            .unwrap_or(&UNKNOWN_FILE.to_string())
            .clone()
    }

    pub fn from_file_and_parse_error(file: String, err: ParseError) -> Self {
        Self {
            inner: err,
            file: Some(file.clone()),
        }
    }
}

impl Deref for ShankParseError {
    type Target = ParseError;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<ParseError> for ShankParseError {
    fn from(err: ParseError) -> Self {
        Self {
            inner: err,
            file: None,
        }
    }
}

pub fn parse_error_into<T: Into<ShankParseError>>(parse_err: T) -> Error {
    let parse_err: ShankParseError = parse_err.into();

    let span = parse_err.span();
    let span_start = stringify_loc(span.start());
    format_err!(
        "Parse Error: {} ({}:{})",
        parse_err.inner.to_string(),
        parse_err.file(),
        span_start,
    )
}

fn stringify_loc(loc: LineColumn) -> String {
    format!("{}:{}", loc.line, loc.column)
}
