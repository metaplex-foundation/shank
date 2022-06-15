use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use shank_idl::{
    extract_idl, idl::Idl, parse_file, ParseIdlConfig, ParseIdlOpts,
};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("accounts")
}

// TODO(thlorenz): Should live in test util crate
pub fn check_or_update_idl(idl: &Idl, json_path: &str) {
    let expected_json_file = fixtures_dir().join(json_path);
    let expected_json =
        read_to_string(&expected_json_file).expect("Unable to read json file");
    let expected_idl: Idl = serde_json::from_str(&expected_json)
        .expect("Unable to parse expected json");

    if std::env::var("UPDATE_IDL").is_ok() {
        let idl_json = idl.try_into_json().unwrap();

        let mut idl_json_file = File::create(&expected_json_file)
            .expect("Unable to create JSON file");

        idl_json_file
            .write_all(idl_json.as_bytes())
            .expect("Unable to write file");
    } else {
        assert_eq!(idl, &expected_idl);
    }
}

#[test]
fn account_from_single_file() {
    let file = fixtures_dir().join("single_file").join("account.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/account.json");
}

#[test]
fn account_from_single_file_complex_types() {
    let file = fixtures_dir().join("single_file").join("complex_types.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    // eprintln!("{}", idl.try_into_json().unwrap());
    check_or_update_idl(&idl, "single_file/complex_types.json");
}

#[test]
fn account_from_single_file_padding() {
    let file = fixtures_dir().join("single_file").join("padding.rs");
    let idl = parse_file(&file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/padding.json");
}

#[test]
fn account_from_crate() {
    let file = fixtures_dir()
        .join("sample_crate")
        .join("src")
        .join("lib.rs");
    let idl = extract_idl(
        file.to_str().unwrap(),
        ParseIdlOpts {
            require_program_address: false,
            ..Default::default()
        },
    )
    .expect("Parsing should not fail")
    .expect("File contains IDL");

    check_or_update_idl(&idl, "sample_crate/idl.json");
}
