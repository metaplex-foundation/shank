use anyhow::Error;
use proc_macro2::LineColumn;
use syn::Error as ParseError;

pub fn parse_error_into<T: Into<ParseError>>(parse_err: T) -> Error {
    let parse_err: ParseError = parse_err.into();
    let span = parse_err.span();
    let span_start = stringify_loc(span.start());

    Error::new(parse_err).context(format!("Parse Error at {}", span_start))
}

fn stringify_loc(loc: LineColumn) -> String {
    format!("{}:{}", loc.line, loc.column)
}
