#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    CreateThing = 256, // u8::MAX + 1,
}
