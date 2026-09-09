use std::convert::TryFrom;

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error as ParseError, GenericArgument, Ident, Path,
    PathArguments, Result as ParseResult, Token, Type, TypePath,
};

const INSTRUCTION_ARGUMENT: &str = "args";

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BuilderArgument {
    pub name: String,
    pub ty: String,
    pub generic_ty: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuilderArguments(pub Vec<BuilderArgument>);

impl BuilderArgument {
    fn is_argument_attr(attr: &Attribute) -> Option<&Attribute> {
        match attr
            .path()
            .get_ident()
            .map(|x| x.to_string().as_str() == INSTRUCTION_ARGUMENT)
        {
            Some(true) => Some(attr),
            _ => None,
        }
    }

    pub fn from_argument_attr(
        attr: &Attribute,
    ) -> ParseResult<BuilderArgument> {
        Self::parse_argument_tokens(attr.parse_args()?)
    }

    fn parse_argument_tokens(
        tokens: ArgumentTokens,
    ) -> ParseResult<BuilderArgument> {
        let ArgumentTokens { name, ty } = tokens;
        let name = name.to_string();
        // type
        match ty {
            Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) => {
                let segment = segments.first().unwrap();

                // check whether we are dealing with a generic type
                let generic_ty = match &segment.arguments {
                    PathArguments::AngleBracketed(arguments) => {
                        if let Some(GenericArgument::Type(Type::Path(ty))) =
                            arguments.args.first()
                        {
                            Some(
                                ty.path
                                    .segments
                                    .first()
                                    .unwrap()
                                    .ident
                                    .to_string(),
                            )
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                Ok(BuilderArgument {
                    name,
                    ty: segment.ident.to_string(),
                    generic_ty,
                })
            }
            ty => Err(ParseError::new_spanned(
                ty,
                "#[args] requires an expression 'name: type'",
            )),
        }
    }
}

/// The `name: Type` inside `#[args(name: Type)]`.
///
/// syn 1 parsed this as a type ascription expression (`ExprType`) which
/// syn 2 no longer supports, so we parse the two parts explicitly.
struct ArgumentTokens {
    name: Ident,
    ty: Type,
}

impl Parse for ArgumentTokens {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name: Ident = input.parse().map_err(|err| {
            ParseError::new(
                err.span(),
                "#[args] requires an expression 'name: type'",
            )
        })?;
        input.parse::<Token![:]>().map_err(|err| {
            ParseError::new(
                err.span(),
                "#[args] requires an expression 'name: type'",
            )
        })?;
        let ty: Type = input.parse()?;
        Ok(ArgumentTokens { name, ty })
    }
}

impl TryFrom<&[Attribute]> for BuilderArguments {
    type Error = ParseError;

    fn try_from(attrs: &[Attribute]) -> ParseResult<Self> {
        let arguments = attrs
            .iter()
            .filter_map(BuilderArgument::is_argument_attr)
            .map(BuilderArgument::from_argument_attr)
            .collect::<ParseResult<Vec<BuilderArgument>>>()?;

        Ok(BuilderArguments(arguments))
    }
}
