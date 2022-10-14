#[derive(ShankInstruction)]
pub enum Instruction {
    #[default_optional_accounts]
    #[account(0, name = "creator", sig)]
    #[account(1, name = "thing", mut, optional)]
    CreateThing(SomeArgs),
    #[account(name = "creator", sig)]
    CloseThing,
}
