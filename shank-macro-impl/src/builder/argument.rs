use std::convert::TryFrom;

use syn::{
    Attribute, Error as ParseError, Expr, ExprPath, ExprType, GenericArgument,
    Path, PathArguments, Result as ParseResult, Type, TypePath,
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
            .path
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

    fn parse_argument_tokens(tokens: ExprType) -> ParseResult<BuilderArgument> {
        let clone = tokens.clone();
        // name
        let name = match *clone.expr {
            Expr::Path(ExprPath {
                path: Path { segments, .. },
                ..
            }) => segments.first().unwrap().ident.to_string(),
            _ => {
                return Err(ParseError::new_spanned(
                    tokens,
                    "#[args] requires an expression 'name: type'",
                ))
            }
        };
        // type
        match *clone.ty {
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
            _ => Err(ParseError::new_spanned(
                tokens,
                "#[args] requires an expression 'name: type'",
            )),
        }
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
