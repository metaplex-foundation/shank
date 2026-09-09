use anyhow::{anyhow, Result};
use cargo_toml;
use serde::Deserialize;
use std::{
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use heck::SnakeCase;

// -----------------
// WithPath
// -----------------
pub struct WithPath<T> {
    inner: T,
    path: PathBuf,
}

// TODO(thlorenz): figure out if we'll actually need this
#[allow(unused)]
impl<T> WithPath<T> {
    pub fn new(inner: T, path: PathBuf) -> Self {
        Self { inner, path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::convert::AsRef<T> for WithPath<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> std::ops::Deref for WithPath<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for WithPath<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// -----------------
// Manifest
// -----------------
#[derive(Debug, Clone, PartialEq)]
pub struct Manifest(cargo_toml::Manifest);

impl Manifest {
    pub fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let path = p.as_ref();
        let content = fs::read_to_string(path)?;

        let mut manifest = match cargo_toml::Manifest::from_str(&content) {
            Ok(manifest) => manifest,
            Err(err) => Self::parse_ignoring_edition(&content)
                .ok_or_else(|| anyhow!(err))?,
        };
        // Discovers implicit data like the lib path and resolves values
        // inherited from a workspace, exactly like
        // `cargo_toml::Manifest::from_path` would.
        manifest.complete_from_path(path)?;

        Ok(Manifest(manifest))
    }

    /// Retries to parse the manifest with the `package.edition` removed.
    ///
    /// The `cargo_toml` crate only accepts the Rust editions it knows about
    /// and thus fails on a manifest which opts into an edition released after
    /// it was published. Shank never consults the edition, it only needs the
    /// package/lib name and version, so it should keep working for any
    /// edition, including future ones.
    ///
    /// Only editions that look like a Rust edition, i.e. a year like `"2030"`,
    /// are ignored. Returns `None` for malformed editions or if the manifest
    /// does not parse for a different reason, so the original error surfaces.
    fn parse_ignoring_edition(content: &str) -> Option<cargo_toml::Manifest> {
        let mut table: toml::Table = content.parse().ok()?;
        let package = table.get_mut("package")?.as_table_mut()?;
        if !package
            .get("edition")?
            .as_str()
            .map_or(false, is_edition_year)
        {
            return None;
        }
        package.remove("edition");
        cargo_toml::Manifest::deserialize(toml::Value::Table(table)).ok()
    }

    pub fn lib_rel_path(&self) -> Option<String> {
        self.lib.as_ref().and_then(|x| x.path.clone())
    }

    pub fn lib_name(&self) -> Result<String> {
        if self.lib.is_some() && self.lib.as_ref().unwrap().name.is_some() {
            Ok(self
                .lib
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                .to_string()
                .to_snake_case())
        } else {
            Ok(self
                .package
                .as_ref()
                .ok_or_else(|| anyhow!("package section not provided"))?
                .name
                .to_string()
                .to_snake_case())
        }
    }

    pub fn version(&self) -> String {
        match &self.package {
            Some(package) => package
                .version
                .get()
                .map(ToString::to_string)
                .unwrap_or_else(|_| "0.0.0".to_string()),
            _ => "0.0.0".to_string(),
        }
    }

    // Climbs each parent directory from a given starting directory until we find a Cargo.toml.
    pub fn discover_from_path(
        start_from: PathBuf,
    ) -> Result<Option<WithPath<Manifest>>> {
        let mut cwd_opt = Some(start_from.as_path());

        while let Some(cwd) = cwd_opt {
            for f in fs::read_dir(cwd)? {
                let p = f?.path();
                if let Some(filename) = p.file_name() {
                    if filename.to_str() == Some("Cargo.toml") {
                        let m = WithPath::new(Manifest::from_path(&p)?, p);
                        return Ok(Some(m));
                    }
                }
            }

            // Not found. Go up a directory level.
            cwd_opt = cwd.parent();
        }

        Ok(None)
    }
}

/// Rust editions are named after a year, e.g. `2021` or `2024`.
fn is_edition_year(edition: &str) -> bool {
    edition.len() == 4 && edition.bytes().all(|b| b.is_ascii_digit())
}

impl Deref for Manifest {
    type Target = cargo_toml::Manifest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FUTURE_EDITION_MANIFEST: &str = r#"
[package]
name = "future-crate"
version = "1.2.3"
edition = "2077"

[lib]
name = "future_lib"
"#;

    #[test]
    fn parse_ignoring_edition_accepts_unknown_edition() {
        assert!(
            cargo_toml::Manifest::from_str(FUTURE_EDITION_MANIFEST).is_err(),
            "cargo_toml itself should reject the unknown edition"
        );
        let manifest = Manifest(
            Manifest::parse_ignoring_edition(FUTURE_EDITION_MANIFEST)
                .expect("should parse when ignoring the edition"),
        );
        assert_eq!(manifest.lib_name().unwrap(), "future_lib");
        assert_eq!(manifest.version(), "1.2.3");
    }

    #[test]
    fn parse_ignoring_edition_rejects_other_errors() {
        assert!(
            Manifest::parse_ignoring_edition("[package]\nname = 1\n").is_none()
        );
        assert!(Manifest::parse_ignoring_edition("not toml at all").is_none());
    }

    #[test]
    fn parse_ignoring_edition_rejects_malformed_editions() {
        for edition in
            ["\"not-an-edition\"", "\"20240\"", "\"24\"", "2024", "true"]
        {
            let manifest = format!(
                "[package]\nname = \"p\"\nversion = \"0.1.0\"\nedition = {}\n",
                edition
            );
            assert!(
                cargo_toml::Manifest::from_str(&manifest).is_err(),
                "cargo_toml should reject edition {}",
                edition
            );
            assert!(
                Manifest::parse_ignoring_edition(&manifest).is_none(),
                "shank should not ignore malformed edition {}",
                edition
            );
        }
    }
}
