#[derive(ShankType)]
pub struct StructTypeWithPadding {
    count: u8,
    #[padding]
    _padding: [u8; 3],
}
