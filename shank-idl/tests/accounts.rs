use std::path::{Path, PathBuf};

use shank_idl::{extract_idl, idl::Idl, parse_file};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("accounts")
}

#[test]
fn account_from_single_file() {
    let file = fixtures_dir().join("single_file").join("account.rs");
    let idl = parse_file(&file, "1.0.0".to_string())
        .expect("Parsing should not fail")
        .expect("File contains IDL");
    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/accounts/single_file/idl.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn account_from_crate() {
    let file = fixtures_dir()
        .join("sample_crate")
        .join("src")
        .join("lib.rs");
    let idl = extract_idl(file.to_str().unwrap())
        .expect("Parsing should not fail")
        .expect("File contains IDL");
    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/accounts/sample_crate/idl.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}
