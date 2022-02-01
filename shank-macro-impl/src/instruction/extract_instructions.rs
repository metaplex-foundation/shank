use crate::parse_result::ShankParseResult;

use super::Instruction;

pub fn extract_instruction_enums<'a>(
    enums: impl Iterator<Item = (String, &'a syn::ItemEnum)>,
) -> ShankParseResult<Vec<Instruction>> {
    let ixs = enums
        .map(Instruction::try_from_item_enum)
        .collect::<ShankParseResult<Vec<Option<Instruction>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>();

    Ok(ixs)
}
