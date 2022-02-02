use anyhow::Result;
use clap::StructOpt;
use shank_cli::Opts;

fn main() -> Result<()> {
    shank_cli::entry(Opts::parse())
}
