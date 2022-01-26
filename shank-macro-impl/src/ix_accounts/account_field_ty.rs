use std::fmt::Debug;

use super::common::{ident_string, tts_to_string};

use syn::{
    spanned::Spanned, Error as ParseError, Result as ParseResult, TypePath,
};

// -----------------
// Account Types
// -----------------
#[derive(Debug, PartialEq)]
pub enum Ty {
    AccountInfo,
    UncheckedAccount,
    ProgramState(ProgramStateTy),
    CpiState(CpiStateTy),
    ProgramAccount(ProgramAccountTy),
    Loader(LoaderTy),
    AccountLoader(AccountLoaderTy),
    CpiAccount(CpiAccountTy),
    Sysvar(SysvarTy),
    Account(AccountTy),
    Program(ProgramTy),
    Signer,
    SystemAccount,
    ProgramData,
}

#[derive(Debug, PartialEq)]
pub enum SysvarTy {
    Clock,
    Rent,
    EpochSchedule,
    Fees,
    RecentBlockhashes,
    SlotHashes,
    SlotHistory,
    StakeHistory,
    Instructions,
    Rewards,
}

#[derive(Debug, PartialEq)]
pub struct ProgramStateTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct CpiStateTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct ProgramAccountTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct CpiAccountTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct AccountLoaderTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct LoaderTy {
    pub account_type_path: TypePath,
}

#[derive(Debug, PartialEq)]
pub struct AccountTy {
    pub account_type_path: TypePath,
    pub boxed: bool,
}

#[derive(Debug, PartialEq)]
pub struct ProgramTy {
    pub account_type_path: TypePath,
}

// -----------------
// Parsers
// -----------------
pub fn parse_account_field_ty(f: &syn::Field) -> ParseResult<Ty> {
    let path = match &f.ty {
        syn::Type::Path(ty_path) => ty_path.path.clone(),
        _ => {
            return Err(ParseError::new(
                f.ty.span(),
                "invalid account type given",
            ))
        }
    };
    let ty = match ident_string(f)?.as_str() {
        "ProgramState" => Ty::ProgramState(parse_program_state(&path)?),
        "CpiState" => Ty::CpiState(parse_cpi_state(&path)?),
        "ProgramAccount" => Ty::ProgramAccount(parse_program_account(&path)?),
        "CpiAccount" => Ty::CpiAccount(parse_cpi_account(&path)?),
        "Sysvar" => Ty::Sysvar(parse_sysvar(&path)?),
        "AccountInfo" => Ty::AccountInfo,
        "UncheckedAccount" => Ty::UncheckedAccount,
        "Loader" => Ty::Loader(parse_program_account_zero_copy(&path)?),
        "AccountLoader" => {
            Ty::AccountLoader(parse_program_account_loader(&path)?)
        }
        "Account" => Ty::Account(parse_account_ty(&path)?),
        "Program" => Ty::Program(parse_program_ty(&path)?),
        "Signer" => Ty::Signer,
        "SystemAccount" => Ty::SystemAccount,
        "ProgramData" => Ty::ProgramData,
        _ => {
            return Err(ParseError::new(
                f.ty.span(),
                "invalid account type given",
            ))
        }
    };

    Ok(ty)
}

fn parse_program_state(path: &syn::Path) -> ParseResult<ProgramStateTy> {
    let account_ident = parse_account(path)?;
    Ok(ProgramStateTy {
        account_type_path: account_ident,
    })
}

fn parse_cpi_state(path: &syn::Path) -> ParseResult<CpiStateTy> {
    let account_ident = parse_account(path)?;
    Ok(CpiStateTy {
        account_type_path: account_ident,
    })
}

fn parse_cpi_account(path: &syn::Path) -> ParseResult<CpiAccountTy> {
    let account_ident = parse_account(path)?;
    Ok(CpiAccountTy {
        account_type_path: account_ident,
    })
}

fn parse_program_account(path: &syn::Path) -> ParseResult<ProgramAccountTy> {
    let account_ident = parse_account(path)?;
    Ok(ProgramAccountTy {
        account_type_path: account_ident,
    })
}

fn parse_program_account_zero_copy(path: &syn::Path) -> ParseResult<LoaderTy> {
    let account_ident = parse_account(path)?;
    Ok(LoaderTy {
        account_type_path: account_ident,
    })
}

fn parse_program_account_loader(
    path: &syn::Path,
) -> ParseResult<AccountLoaderTy> {
    let account_ident = parse_account(path)?;
    Ok(AccountLoaderTy {
        account_type_path: account_ident,
    })
}

fn parse_account_ty(path: &syn::Path) -> ParseResult<AccountTy> {
    let account_type_path = parse_account(path)?;
    let boxed = tts_to_string(&path)
        .replace(' ', "")
        .starts_with("Box<Account<");
    Ok(AccountTy {
        account_type_path,
        boxed,
    })
}

fn parse_program_ty(path: &syn::Path) -> ParseResult<ProgramTy> {
    let account_type_path = parse_account(path)?;
    Ok(ProgramTy { account_type_path })
}

fn parse_account(mut path: &syn::Path) -> ParseResult<syn::TypePath> {
    if tts_to_string(path)
        .replace(' ', "")
        .starts_with("Box<Account<")
    {
        let segments = &path.segments[0];
        match &segments.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                // Expected: <'info, MyType>.
                if args.args.len() != 1 {
                    return Err(ParseError::new(
                        args.args.span(),
                        "bracket arguments must be the lifetime and type",
                    ));
                }
                match &args.args[0] {
                    syn::GenericArgument::Type(syn::Type::Path(ty_path)) => {
                        path = &ty_path.path;
                    }
                    _ => {
                        return Err(ParseError::new(
                            args.args[1].span(),
                            "first bracket argument must be a lifetime",
                        ))
                    }
                }
            }
            _ => {
                return Err(ParseError::new(
                    segments.arguments.span(),
                    "expected angle brackets with a lifetime and type",
                ))
            }
        }
    }

    let segments = &path.segments[0];
    match &segments.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            // Expected: <'info, MyType>.
            if args.args.len() != 2 {
                return Err(ParseError::new(
                    args.args.span(),
                    "bracket arguments must be the lifetime and type",
                ));
            }
            match &args.args[1] {
                syn::GenericArgument::Type(syn::Type::Path(ty_path)) => {
                    Ok(ty_path.clone())
                }
                _ => Err(ParseError::new(
                    args.args[1].span(),
                    "first bracket argument must be a lifetime",
                )),
            }
        }
        _ => Err(ParseError::new(
            segments.arguments.span(),
            "expected angle brackets with a lifetime and type",
        )),
    }
}
fn parse_sysvar(path: &syn::Path) -> ParseResult<SysvarTy> {
    let segments = &path.segments[0];
    let account_ident = match &segments.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            // Expected: <'info, MyType>.
            if args.args.len() != 2 {
                return Err(ParseError::new(
                    args.args.span(),
                    "bracket arguments must be the lifetime and type",
                ));
            }
            match &args.args[1] {
                syn::GenericArgument::Type(syn::Type::Path(ty_path)) => {
                    // TODO: allow segmented paths.
                    if ty_path.path.segments.len() != 1 {
                        return Err(ParseError::new(
                            ty_path.path.span(),
                            "segmented paths are not currently allowed",
                        ));
                    }
                    let path_segment = &ty_path.path.segments[0];
                    path_segment.ident.clone()
                }
                _ => {
                    return Err(ParseError::new(
                        args.args[1].span(),
                        "first bracket argument must be a lifetime",
                    ))
                }
            }
        }
        _ => {
            return Err(ParseError::new(
                segments.arguments.span(),
                "expected angle brackets with a lifetime and type",
            ))
        }
    };
    let ty = match account_ident.to_string().as_str() {
        "Clock" => SysvarTy::Clock,
        "Rent" => SysvarTy::Rent,
        "EpochSchedule" => SysvarTy::EpochSchedule,
        "Fees" => SysvarTy::Fees,
        "RecentBlockhashes" => SysvarTy::RecentBlockhashes,
        "SlotHashes" => SysvarTy::SlotHashes,
        "SlotHistory" => SysvarTy::SlotHistory,
        "StakeHistory" => SysvarTy::StakeHistory,
        "Instructions" => SysvarTy::Instructions,
        "Rewards" => SysvarTy::Rewards,
        _ => {
            return Err(ParseError::new(
                account_ident.span(),
                "invalid sysvar provided",
            ))
        }
    };
    Ok(ty)
}
