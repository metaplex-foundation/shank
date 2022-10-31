#[derive(ShankInstruction)]
pub enum Instruction {
    #[idl_instruction(Create)]
    Create,
    #[idl_instruction(CreateBuffer)]
    CreateBuffer,
}
