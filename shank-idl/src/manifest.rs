use anyhow::{anyhow, Result};
use cargo_toml::{self, Package};
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
        cargo_toml::Manifest::from_path(p)
            .map(Manifest)
            .map_err(Into::into)
    }

    pub fn lib_rel_path(&self) -> Option<String> {
        self.lib
            .as_ref()
            .map(|x| x.path.clone())
            .flatten()
            .to_owned()
    }

    pub fn version(&self) -> String {
        match &self.package {
            Some(package) => package.version.to_string(),
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

impl Deref for Manifest {
    type Target = cargo_toml::Manifest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
