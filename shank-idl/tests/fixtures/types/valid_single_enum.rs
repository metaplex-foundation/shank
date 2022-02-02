#[derive(BorshSerialize)]
pub enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
    White,
}
