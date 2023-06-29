#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", signer)]
    #[account(1, name = "thing", writable)]
    CreateThing(SomeArgs),
    #[account(name = "creator", optional_signer)]
    CloseThing,
}
