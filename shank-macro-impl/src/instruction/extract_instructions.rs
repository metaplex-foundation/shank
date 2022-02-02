use super::Instruction;
use syn::Result as ParseResult;

pub fn extract_instruction_enums<'a>(
    enums: impl Iterator<Item = &'a syn::ItemEnum>,
) -> ParseResult<Vec<Instruction>> {
    let ixs = enums
        .map(|x| Instruction::try_from_item_enum(x, false))
        .collect::<ParseResult<Vec<Option<Instruction>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>();

    Ok(ixs)
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemEnum;

    use super::*;

    fn parse_instructions(
        codes: Vec<TokenStream>,
    ) -> ParseResult<Vec<Instruction>> {
        let item_enums = codes
            .into_iter()
            .map(syn::parse2::<ItemEnum>)
            .collect::<ParseResult<Vec<ItemEnum>>>()
            .expect("Should parse ItemEnum successfully");

        extract_instruction_enums(item_enums.iter())
    }

    fn instruction_valid() -> TokenStream {
        quote! {
            #[derive(ShankInstruction)]
            pub enum InstructionValid {
                #[account(0, name = "creator", sig)]
                CreateThing,
                CloseThing
            }
        }
    }
    fn instruction_missing_derive() -> TokenStream {
        quote! {
            pub enum InstructionMissingDerive {
                #[account(0, name = "creator", sig)]
                CreateThing,
                CloseThing
            }
        }
    }

    fn instruction_invalid_account_name() -> TokenStream {
        quote! {
            #[derive(ShankInstruction)]
            pub enum InstructionInvalidAccountName {
                #[account(naaame = "creator", sig)]
                CreateThing
            }
        }
    }

    fn instruction_unknown_account_attr() -> TokenStream {
        quote! {
            #[derive(ShankInstruction)]
            pub enum InstructionUnknownAccountAttr {
                #[account(name = "creator", unknown)]
                CreateThing
            }
        }
    }

    fn instruction_invalid_account_idx() -> TokenStream {
        quote! {
            #[derive(ShankInstruction)]
            pub enum InstructionUnknownAccountAttr {
                #[account(1, name = "creator")]
                CreateThing
            }
        }
    }

    #[test]
    fn extract_valid_instructions() {
        let ixs = parse_instructions(vec![
            instruction_valid(),
            instruction_missing_derive(),
        ])
        .expect("Should parse fine");

        assert_eq!(ixs.len(), 1, "extracts the one valid instruction")
    }

    #[test]
    fn extract_valid_instruction_and_invalid_account_name() {
        let res = parse_instructions(vec![
            instruction_valid(),
            instruction_invalid_account_name(),
        ]);
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("Only desc/description or name"));
    }

    #[test]
    fn extract_valid_instruction_and_unknown_account_attr() {
        let res = parse_instructions(vec![
            instruction_unknown_account_attr(),
            instruction_valid(),
        ]);
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("Invalid/unknown account meta configuration"));
    }

    #[test]
    fn extract_valid_instruction_and_invalid_account_idx() {
        let res = parse_instructions(vec![
            instruction_invalid_account_idx(),
            instruction_valid(),
        ]);
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("Account index 1 does not match"));
    }
}
