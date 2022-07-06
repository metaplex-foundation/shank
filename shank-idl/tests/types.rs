use std::path::{Path, PathBuf};

use shank_idl::{idl::Idl, parse_file, ParseIdlConfig};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("types")
}

#[test]
fn type_valid_single_struct() {
    let file = fixtures_dir().join("valid_single_struct.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/types/valid_single_struct.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn type_valid_single_emum() {
    let file = fixtures_dir().join("valid_single_enum.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/types/valid_single_enum.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn type_valid_single_data_emum() {
    let file = fixtures_dir().join("valid_single_data_enum.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/types/valid_single_data_enum.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn type_valid_multiple() {
    let file = fixtures_dir().join("valid_multiple.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");
    // eprintln!("{}", serde_json::to_string_pretty(&idl).unwrap());

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/types/valid_multiple.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn type_invalid_single() {
    let file = fixtures_dir().join("invalid_single.rs");
    assert!(
        parse_file(&file, &ParseIdlConfig::optional_program_address()).is_err()
    )
}
