/// Context wrapper that provides access to accounts and remaining accounts
pub struct Context<'a, T, U = ()> {
    pub accounts: T,
    pub remaining_accounts: &'a [U],
}