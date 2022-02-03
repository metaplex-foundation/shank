#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    CreateThing = 4294967296, // u32::MAX + 1,
}
