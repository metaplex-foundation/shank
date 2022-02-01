#[derive(ShankInstruction)]
pub enum Instruction {
    // Misspelled sig
    #[account(0, name = "creator", sg)]
    CreateThing,
}
