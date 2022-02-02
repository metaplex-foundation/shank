use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Parser;
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
        #[clap(short, long)]
        crate_root: String,
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

pub fn idl(idl_json: Option<String>, crate_root: String) -> Result<()> {
    let crate_root = Path::new(&crate_root);
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
