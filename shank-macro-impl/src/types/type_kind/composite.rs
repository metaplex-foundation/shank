use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Composite {
    Vec,
    Array(usize),
    Option,
    HashMap,
    Custom(String),
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Composite::Vec => write!(f, "Composite::Vec"),
            Composite::Array(size) => write!(f, "Composite::Array({})", size),
            Composite::Option => write!(f, "Composite::Option"),
            Composite::HashMap => write!(f, "Composite::HashMap"),
            Composite::Custom(name) => {
                write!(f, "Composite::Custom(\"{}\")", name)
            }
        }
    }
}
