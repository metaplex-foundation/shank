#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    #[account(1, name = "thing", mut)]
    CreateThing,
    #[account(name = "original_creator", sig)]
    CloseThing,
}
