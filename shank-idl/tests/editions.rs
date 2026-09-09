//! Verifies that IDL extraction works for crates of any Rust edition.
//!
//! Older editions are covered by the remaining fixtures (they opt into the
//! 2018 edition), so this file focuses on the 2024 edition and on editions
//! shank's manifest parser does not know about yet.
use std::{
    fs::{self, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use shank_idl::{extract_idl, idl::Idl, ParseIdlOpts};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("editions")
}

fn check_or_update_idl(idl: &Idl, json_path: &str) {
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

fn extract_crate_idl(lib_rs: &Path) -> Idl {
    extract_idl(
        lib_rs.to_str().unwrap(),
        ParseIdlOpts {
            require_program_address: false,
            ..Default::default()
        },
    )
    .expect("Parsing should not fail")
    .expect("File contains IDL")
}

#[test]
fn edition_2024_crate() {
    let file = fixtures_dir()
        .join("edition_2024")
        .join("src")
        .join("lib.rs");
    let idl = extract_crate_idl(&file);

    assert_eq!(idl.name, "edition_2024_crate");
    assert_eq!(idl.version, "0.2.0");
    check_or_update_idl(&idl, "edition_2024/idl.json");
}

/// The manifest parser shank uses only knows the editions that existed when
/// it was released. Since shank never needs the edition, a crate opting into
/// a newer one must still work.
#[test]
fn unknown_future_edition_crate() {
    let crate_dir = std::env::temp_dir()
        .join(format!("shank_idl_future_edition_{}", std::process::id()));
    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir).expect("Unable to create fixture dir");
    fs::write(
        crate_dir.join("Cargo.toml"),
        r#"
[package]
name = "future_edition_crate"
version = "3.2.1"
edition = "2077"

[dependencies]
"#,
    )
    .expect("Unable to write Cargo.toml");
    let lib_rs = src_dir.join("lib.rs");
    fs::write(
        &lib_rs,
        r#"
use shank::ShankAccount;

#[derive(ShankAccount)]
pub struct Counter {
    pub count: u64,
}
"#,
    )
    .expect("Unable to write lib.rs");

    let idl = extract_crate_idl(&lib_rs);
    fs::remove_dir_all(&crate_dir).ok();

    assert_eq!(idl.name, "future_edition_crate");
    assert_eq!(idl.version, "3.2.1");
    assert_eq!(idl.accounts.len(), 1);
    assert_eq!(idl.accounts[0].name, "Counter");
}
