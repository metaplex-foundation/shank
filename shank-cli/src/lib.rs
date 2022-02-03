use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use log::{debug, info, trace, warn};
use shank_idl::{extract_idl, manifest::Manifest};

#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Idl {
        /// Output file for the IDL JSON.
        #[clap(short, long)]
        idl_json: Option<String>,

        /// Directory of program crate for which to generate the IDL.
        #[clap(short = 'r', long)]
        crate_root: Option<String>,
    },
}

pub fn entry(opts: Opts) -> Result<()> {
    match opts.command {
        Command::Idl {
            idl_json,
            crate_root,
        } => idl(idl_json, crate_root),
    }
}

pub fn try_resolve_path(p: Option<String>, label: &str) -> Result<PathBuf> {
    let p = match p {
        Some(crate_root) => Ok(Path::new(&crate_root).to_path_buf()),
        None => {
            debug!("No {} provided, assuming current dir", label);
            std::env::current_dir()
        }
    }?;

    let p = if p.is_absolute() {
        Ok(p.to_path_buf())
    } else {
        debug!("{} is relative, resolving from current dir", label);
        std::env::current_dir().map(|x| x.join(p))
    }?;

    Ok(p)
}

pub fn idl(idl_json: Option<String>, crate_root: Option<String>) -> Result<()> {
    let crate_root = try_resolve_path(crate_root, "crate_root")?;

    let cargo_toml = crate_root.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow!(
            "Did not find Cargo.toml at the path: {}",
            crate_root.display()
        ));
    }

    let lib_rel_path = Manifest::from_path(&cargo_toml)?
        .lib_rel_path()
        .ok_or(anyhow!("Program needs to be a lib"))?;

    let lib_full_path_str = crate_root.join(lib_rel_path);
    let lib_full_path = lib_full_path_str.to_str().ok_or(anyhow!("Invalid Path"))?;

    let idl = extract_idl(lib_full_path);
    eprintln!("{:#?}", idl);
    Ok(())
}
