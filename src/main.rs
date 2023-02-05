use std::{collections::HashSet, path::PathBuf};

use clap::Parser;

use configure_semantic_release_assets::{SemanticReleaseConfiguration, WriteTo};

const SEMANTIC_RELEASE_MANIFEST_PATH: &'static str = ".releaserc.json";

#[derive(Debug, Parser)]
enum Subcommand {
    /// Trim release assets to a whitelist
    Whitelist {
        /// Whitelist of release assets
        #[arg()]
        whitelist: String,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, default_value = SEMANTIC_RELEASE_MANIFEST_PATH)]
    config: PathBuf,

    /// Edit file in-place
    #[arg(long, action)]
    in_place: bool,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    let mut configuration = SemanticReleaseConfiguration::read_from_file(&cli.config)?;

    match cli.subcommand {
        Subcommand::Whitelist { whitelist } => {
            configuration.apply_whitelist(HashSet::from_iter(
                whitelist.split_whitespace().map(|s| s.to_owned()),
            ));
        }
    }

    match cli.in_place {
        true => configuration.write_if_modified(WriteTo::InPlace)?,
        false => configuration.write_if_modified(WriteTo::Stdout)?,
    };

    Ok(())
}
