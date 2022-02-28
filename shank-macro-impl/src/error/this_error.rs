use std::convert::TryFrom;

use anyhow::{format_err, Result};

use crate::{parsed_enum::ParsedEnum, parsers::get_derive_attr};
use syn::Result as ParseResult;

use super::{ProgramError, ProgramErrors, DERIVE_THIS_ERROR_ATTR};

fn filter_this_error_enums<'a>(
    enums: impl Iterator<Item = &'a syn::ItemEnum>,
) -> Vec<&'a syn::ItemEnum> {
    enums
        .filter_map(|item_enum| {
            get_derive_attr(&item_enum.attrs, DERIVE_THIS_ERROR_ATTR)
                .map(|_| item_enum)
        })
        .collect()
}

fn extract_this_error_enums<'a>(
    enums: impl Iterator<Item = &'a syn::ItemEnum>,
) -> Result<Vec<ParsedEnum>> {
    let mut error_enums = Vec::new();

    for x in filter_this_error_enums(enums) {
        let enm = ParsedEnum::try_from(x).map_err(|err| {
            format_err!(
                "Encountered an error parsing {} this_error enum.\n{}",
                x.ident,
                err
            )
        })?;
        error_enums.push(enm);
    }
    Ok(error_enums)
}

pub fn extract_this_errors<'a>(
    enums: impl Iterator<Item = &'a syn::ItemEnum>,
) -> Result<Vec<ProgramError>> {
    let this_error_enums = extract_this_error_enums(enums)?;
    let program_errors = this_error_enums
        .iter()
        .map(ProgramErrors::try_from)
        .collect::<ParseResult<Vec<ProgramErrors>>>()?
        .into_iter()
        .map(|x| x.0)
        .flatten()
        .collect::<Vec<ProgramError>>();
    Ok(program_errors)
}
