#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    CreateThing = u64::MAX + 1, // u8::MAX + 1,
}
