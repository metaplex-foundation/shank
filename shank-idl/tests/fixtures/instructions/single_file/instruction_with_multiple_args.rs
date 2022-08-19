#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    CloseThing(Option<u8>, ComplexArgs, ComplexArgs),
}
