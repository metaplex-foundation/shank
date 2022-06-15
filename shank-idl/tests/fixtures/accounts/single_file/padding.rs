#[derive(ShankAccount)]
pub struct StructAccountWithPadding {
    count: u8,
    #[padding]
    _padding: [u8; 3],
}
