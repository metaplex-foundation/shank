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
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/account.json");
}

#[test]
fn account_from_single_file_complex_types() {
    let file = fixtures_dir().join("single_file").join("complex_types.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    // eprintln!("{}", idl.try_into_json().unwrap());
    check_or_update_idl(&idl, "single_file/complex_types.json");
}

#[test]
fn account_from_single_file_padding() {
    let file = fixtures_dir().join("single_file").join("padding.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/padding.json");
}

#[test]
fn account_from_single_file_idl_type() {
    let file = fixtures_dir().join("single_file").join("idl_type.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/idl_type.json");
}

#[test]
fn account_from_single_file_field_attributes() {
    let file = fixtures_dir()
        .join("single_file")
        .join("field_attributes.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/field_attributes.json");
}

#[test]
fn account_from_single_file_podded_types() {
    let file = fixtures_dir().join("single_file").join("podded_types.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    check_or_update_idl(&idl, "single_file/podded_types.json");
}

#[test]
fn account_from_single_file_pod_option_enum_sentinel() {
    let file = fixtures_dir()
        .join("single_file")
        .join("pod_option_enum_sentinel.rs");
    let idl = parse_file(file, &ParseIdlConfig::optional_program_address())
        .expect("Parsing should not fail for enum with pod_sentinel")
        .expect("File contains IDL");

    // Verify the Condition type has its sentinel preserved
    let condition_type = idl
        .types
        .iter()
        .find(|t| t.name == "Condition")
        .expect("Should have Condition type");
    assert!(
        condition_type.pod_sentinel.is_some(),
        "Condition enum should have pod_sentinel"
    );
    assert_eq!(
        condition_type.pod_sentinel.as_ref().unwrap(),
        &vec![255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        "Sentinel should match the bytes from #[pod_sentinel(255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)]"
    );

    // Verify the account field uses fixedSizeOption with the sentinel
    let account = idl
        .accounts
        .iter()
        .find(|a| a.name == "AccountWithEnumPodOption")
        .expect("Should have AccountWithEnumPodOption");
    match &account.ty {
        shank_idl::idl_type_definition::IdlTypeDefinitionTy::Struct {
            fields,
        } => {
            let field = fields
                .iter()
                .find(|f| f.name == "optionalCondition")
                .expect("Should have optionalCondition field");
            match &field.ty {
                shank_idl::idl_type::IdlType::FixedSizeOption {
                    sentinel,
                    ..
                } => {
                    assert!(
                        sentinel.is_some(),
                        "FixedSizeOption should have sentinel populated"
                    );
                    assert_eq!(
                        sentinel.as_ref().unwrap(),
                        &vec![
                            255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ]
                    );
                }
                other => panic!("Expected FixedSizeOption, got {:?}", other),
            }
        }
        _ => panic!("Expected struct"),
    }
}

#[test]
fn account_from_single_file_pod_option_missing_sentinel() {
    let file = fixtures_dir()
        .join("single_file")
        .join("pod_option_missing_sentinel.rs");
    let result = parse_file(file, &ParseIdlConfig::optional_program_address());

    assert!(
        result.is_err(),
        "Expected validation error for missing sentinel"
    );
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("PodOption validation errors"),
        "Error message should mention PodOption validation: {}",
        err_msg
    );
    assert!(
        err_msg.contains("CustomTypeWithoutSentinel"),
        "Error message should mention the custom type: {}",
        err_msg
    );
    assert!(
        err_msg.contains("does not define #[pod_sentinel(...)"),
        "Error message should mention missing pod_sentinel: {}",
        err_msg
    );
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
