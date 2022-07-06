use std::path::{Path, PathBuf};

use shank_idl::{idl::Idl, parse_file, ParseIdlConfig};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("errors")
}

#[test]
fn errors_this_error() {
    let file = fixtures_dir().join("this_error.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    // eprintln!("{}", idl.try_into_json().unwrap());

    let expected_idl: Idl =
        serde_json::from_str(include_str!("./fixtures/errors/this_error.json"))
            .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn errors_this_error_custom_codes() {
    let file = fixtures_dir().join("this_error_custom_codes.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/errors/this_error_custom_codes.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}
