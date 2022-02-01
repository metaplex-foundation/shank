#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    #[account(1, name = "thing", mut)]
    CreateThing(SomeArgs),
    #[account(name = "creator", sig)]
    CloseThing(Option<u8>),
}
