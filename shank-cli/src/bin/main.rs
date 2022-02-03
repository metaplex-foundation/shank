use anyhow::Result;
use clap::StructOpt;
use shank_cli::Opts;

use fern::colors::{Color, ColoredLevelConfig};

fn main() -> Result<()> {
    setup_logging();
    shank_cli::entry(Opts::parse())
}

fn setup_logging() {
    let colors = ColoredLevelConfig::new().debug(Color::BrightBlue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "shank {} {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .apply()
        .unwrap();
}
