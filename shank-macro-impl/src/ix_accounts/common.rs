use syn::{spanned::Spanned, Error as ParseError, Result as ParseResult};

pub fn ident_string(f: &syn::Field) -> ParseResult<String> {
    let path = match &f.ty {
        syn::Type::Path(ty_path) => ty_path.path.clone(),
        _ => return Err(ParseError::new(f.ty.span(), "invalid type")),
    };
    if tts_to_string(&path)
        .replace(' ', "")
        .starts_with("Box<Account<")
    {
        return Ok("Account".to_string());
    }
    // TODO: allow segmented paths.
    if path.segments.len() != 1 {
        return Err(ParseError::new(
            f.ty.span(),
            "segmented paths are not currently allowed",
        ));
    }

    let segments = &path.segments[0];
    Ok(segments.ident.to_string())
}

pub fn tts_to_string<T: quote::ToTokens>(item: T) -> String {
    let mut tts = proc_macro2::TokenStream::new();
    item.to_tokens(&mut tts);
    tts.to_string()
}
