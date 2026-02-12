use shank::{ShankAccount, ShankType};

/// Generic PodOption type (normally from podded crate)
pub struct PodOption<T>(pub T);

/// Enum WITH pod_sentinel - should work with PodOption
#[derive(ShankType)]
#[pod_sentinel(255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)]
pub enum Condition {
    TimeAbsolute { time: i64 },
    TimeRelative { offset: i64 },
}

/// Account using PodOption with an enum that has a sentinel
#[derive(ShankAccount)]
pub struct AccountWithEnumPodOption {
    /// This should succeed because Condition has pod_sentinel
    pub optional_condition: PodOption<Condition>,
}
