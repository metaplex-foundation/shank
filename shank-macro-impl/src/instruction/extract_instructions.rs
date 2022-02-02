use super::Instruction;
use syn::Result as ParseResult;

pub fn extract_instruction_enums<'a>(
    enums: impl Iterator<Item = (String, &'a syn::ItemEnum)>,
) -> ParseResult<Vec<Instruction>> {
    let ixs = enums
        .map(Instruction::try_from_item_enum)
        .collect::<ParseResult<Vec<Option<Instruction>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>();

    Ok(ixs)
}
