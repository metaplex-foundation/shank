#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    #[account(1, name = "thing", mut)]
    CreateThing {
        some_args: SomeArgs,
        other_args: OtherArgs,
    },
    #[account(0, name = "creator", sig)]
    CloseThing(Option<u8>),
}
