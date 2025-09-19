#[test]
fn expand_basic_accounts() {
    macrotest::expand("tests/expand/basic_accounts.rs");
}

#[test] 
fn expand_optional_accounts() {
    macrotest::expand("tests/expand/optional_accounts.rs");
}

#[test]
fn expand_complex_constraints() {
    macrotest::expand("tests/expand/complex_constraints.rs");
}

#[test]
fn expand_no_constraints() {
    macrotest::expand("tests/expand/no_constraints.rs");
}

#[test]
fn expand_custom_lifetime() {
    macrotest::expand("tests/expand/custom_lifetime.rs");
}

#[test]
fn expand_empty_struct() {
    macrotest::expand("tests/expand/empty_struct.rs");
}