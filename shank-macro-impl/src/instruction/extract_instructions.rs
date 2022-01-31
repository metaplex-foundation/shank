use syn::Result as ParseResult;

use super::Instruction;

pub fn extract_instruction_enums<'a>(
    enums: impl Iterator<Item = &'a syn::ItemEnum>,
) -> ParseResult<Vec<Instruction>> {
    let ixs = enums
        .map(Instruction::try_from_item_enum)
        .collect::<ParseResult<Vec<Option<Instruction>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>();

    Ok(ixs)
}
