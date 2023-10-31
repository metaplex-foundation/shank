use std::path::{Path, PathBuf};

use shank_idl::{idl::Idl, parse_file, ParseIdlConfig};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("macros")
}

#[test]
fn macro_valid_program_id() {
    let file = fixtures_dir().join("program_id_valid.rs");
    let idl = parse_file(file, &ParseIdlConfig::default())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/macros/program_id_valid.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn macro_missing_program_id() {
    let file = fixtures_dir().join("program_id_missing.rs");
    let err = parse_file(file, &ParseIdlConfig::default())
        .expect_err("Should fail")
        .to_string();
    assert!(err.contains("Could not find"));
    assert!(err.contains("declare_id"));
}

#[test]
fn macro_missing_program_id_not_required() {
    let file = fixtures_dir().join("program_id_missing.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/macros/program_id_missing.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}
