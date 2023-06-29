#[derive(ShankInstruction)]
pub enum Instruction {
    #[legacy_optional_accounts_strategy]
    #[account(0, name = "creator", sig)]
    #[account(1, name = "thing", mut, optional)]
    CreateThing(SomeArgs),
    #[account(name = "creator", sig)]
    CloseThing,
}
