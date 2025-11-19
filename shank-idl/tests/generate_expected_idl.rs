// Temporary test to generate expected IDL JSON files
use shank_idl::{parse_file, ParseIdlConfig};
use std::path::Path;

#[test]
#[ignore] // This is just for generating the expected files, ignore in normal runs
fn generate_simple_accounts_struct_idl() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("instructions")
        .join("single_file")
        .join("instruction_with_simple_accounts_struct.rs");

    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    println!("Simple Accounts Struct IDL:");
    println!("{}", idl.try_into_json().unwrap());
}

#[test]
#[ignore] // This is just for generating the expected files, ignore in normal runs
fn generate_accounts_struct_idl() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("instructions")
        .join("single_file")
        .join("instruction_with_accounts_struct.rs");

    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    println!("Accounts Struct IDL:");
    println!("{}", idl.try_into_json().unwrap());
}

#[test]
#[ignore] // This is just for generating the expected files, ignore in normal runs
fn generate_complex_accounts_struct_idl() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("instructions")
        .join("single_file")
        .join("instruction_with_complex_accounts_struct.rs");

    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    println!("Complex Accounts Struct IDL:");
    println!("{}", idl.try_into_json().unwrap());
}
