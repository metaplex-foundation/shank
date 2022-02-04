use std::path::{Path, PathBuf};

use shank_idl::{idl::Idl, parse_file, ParseIdlConfig};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("instructions")
}

#[test]
fn instruction_from_single_file_no_args() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_no_args.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    // eprintln!("{}", idl.try_into_json().unwrap());

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/instructions/single_file/instruction_no_args.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn instruction_from_single_file_with_args() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_with_args.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/instructions/single_file/instruction_with_args.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn instruction_from_single_file_invalid_attr() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_invalid_attr.rs");
    let res = parse_file(&file, &ParseIdlConfig::optional_program_address());

    let err = res.unwrap_err();
    let source_string = err.source().unwrap().to_string();
    assert!(source_string.contains("Invalid"));
    assert!(source_string.contains("account meta configuration"));
}

#[test]
fn instruction_from_single_file_invalid_discriminant() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_invalid_discriminant.rs");
    let res = parse_file(&file, &ParseIdlConfig::optional_program_address());

    let err = res.unwrap_err().to_string();
    assert!(err.contains("discriminants have to be <= u32::MAX"));
    assert!(err.contains("discriminant of variant 'CreateThing' is 4294967296"));
}
