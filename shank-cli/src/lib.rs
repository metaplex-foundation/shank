use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, format_err, Result};
use clap::Parser;
use log::{debug, info};
use shank_idl::{extract_idl, manifest::Manifest, ParseIdlOpts};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[clap(version = VERSION)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Idl {
        /// Output directory for the IDL JSON.
        #[clap(short, long, default_value = "idl")]
        out_dir: String,

        /// Directory of program crate for which to generate the IDL.
        #[clap(short = 'r', long)]
        crate_root: Option<String>,

        /// Manually specify and override the address in the IDL
        #[clap(short = 'p', long)]
        program_id: Option<String>,
    },
}

pub fn entry(opts: Opts) -> Result<()> {
    match opts.command {
        Command::Idl {
            out_dir,
            crate_root,
            program_id,
        } => idl(out_dir, crate_root, program_id),
    }
}

pub fn try_resolve_path(p: Option<String>, label: &str) -> Result<PathBuf> {
    let p = match p {
        Some(crate_root) => Ok(Path::new(&crate_root).to_path_buf()),
        None => {
            debug!("No {} provided, assuming in current dir", label);
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

pub fn idl(
    out_dir: String,
    crate_root: Option<String>,
    program_id: Option<String>,
) -> Result<()> {
    // Resolve input and output directories
    let crate_root = try_resolve_path(crate_root, "crate_root")?;
    let out_dir = try_resolve_path(Some(out_dir), "out_dir")?;
    fs::create_dir_all(&out_dir).map_err(|err| {
        format_err!(
            "Unable to create out_dir ({}), {}",
            &out_dir.display(),
            err
        )
    })?;

    // Resolve info about lib for which we generate IDL
    let cargo_toml = crate_root.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow!(
            "Did not find Cargo.toml at the path: {}",
            crate_root.display()
        ));
    }
    let manifest = Manifest::from_path(&cargo_toml)?;
    let lib_rel_path = manifest
        .lib_rel_path()
        .ok_or(anyhow!("Program needs to be a lib"))?;

    let lib_full_path_str = crate_root.join(lib_rel_path);
    let lib_full_path =
        lib_full_path_str.to_str().ok_or(anyhow!("Invalid Path"))?;

    // Extract IDL and convert to JSON
    let mut opts = ParseIdlOpts::default();
    opts.program_address_override = program_id;
    let idl = extract_idl(lib_full_path, opts)?
        .ok_or(anyhow!("No IDL could be extracted"))?;
    let idl_json = idl.try_into_json()?;

    // Write to JSON file
    let name = manifest.lib_name()?;
    let idl_json_path = out_dir.join(format!("{}.json", name));
    let mut idl_json_file = File::create(&idl_json_path)?;
    info!("Writing IDL to {}", &idl_json_path.display());

    idl_json_file.write_all(idl_json.as_bytes())?;

    Ok(())
}
