use shank::{ShankAccount, ShankType};

/// Generic PodOption type (normally from podded crate)
pub struct PodOption<T>(pub T);

/// Custom type WITHOUT pod_sentinel - this should cause an error when used with PodOption
#[derive(ShankType)]
pub struct CustomTypeWithoutSentinel {
    pub value: u32,
}

/// Account using PodOption with a custom type that lacks a sentinel
#[derive(ShankAccount)]
pub struct AccountWithMissingSentinel {
    /// This should trigger a validation error
    pub pod_option_no_sentinel: PodOption<CustomTypeWithoutSentinel>,
}
