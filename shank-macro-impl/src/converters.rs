use anyhow::Error;
use syn::Error as ParseError;

pub fn parse_error_into<T: Into<ParseError>>(parse_err: T) -> Error {
    let parse_err: ParseError = parse_err.into();
    Error::new(parse_err)
        .context(format!("[ParseError] Run `cargo build` or `cargo check` in the program crate root for more details."))
}
