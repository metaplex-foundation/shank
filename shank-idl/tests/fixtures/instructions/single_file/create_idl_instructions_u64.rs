#[derive(ShankInstruction)]
#[discriminator_size(8)]
pub enum Instruction {
    #[idl_instruction(Create)]
    Create,
    #[idl_instruction(CreateBuffer)]
    CreateBuffer,
    #[idl_instruction(SetBuffer)]
    SetBuffer,
    #[idl_instruction(SetAuthority)]
    SetAuthority,
    #[idl_instruction(Write)]
    Write,
}
