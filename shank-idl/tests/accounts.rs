use std::path::{Path, PathBuf};

use shank_idl::parse_file;

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("accounts")
}

#[test]
fn account_from_single_file() {
    let file = fixtures_dir().join("single_file").join("account.rs");
    let idl = parse_file(&file, "1.0.0".to_string()).unwrap();

    eprintln!("{}", serde_json::to_string_pretty(&idl).unwrap());
}
