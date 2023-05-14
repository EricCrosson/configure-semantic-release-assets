use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub(crate) struct Cli {
    /// Directory in which to search for semantic-release manifest
    #[arg(long, default_value = ".")]
    pub directory: PathBuf,

    /// Edit file in-place
    #[arg(long, action)]
    pub in_place: bool,

    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
    /// Trim release assets to a whitelist
    Whitelist {
        /// Whitelist of release assets
        #[arg()]
        whitelist: Vec<String>,
    },
}
