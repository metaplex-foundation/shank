#[derive(ShankInstruction)]
pub enum Instruction {
    #[idl_instruction(Create)]
    Create,
    #[idl_instruction(CreateBuffer)]
    CreateBuffer,
    #[idl_instruction(SetBuffer)]
    SetBuffer,
    #[idl_instruction(SetAuthority)]
    SetAuthority,
}
