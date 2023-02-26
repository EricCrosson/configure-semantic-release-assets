use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use clap::Parser;

use configure_semantic_release_assets::{SemanticReleaseConfiguration, WriteTo};
use find_semantic_release_config::find_semantic_release_configuration;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>;

const SUPPORTED_FILE_TYPES: &[&str] = &["json"];

#[derive(Debug, Parser)]
enum Subcommand {
    /// Trim release assets to a whitelist
    Whitelist {
        /// Whitelist of release assets
        #[arg()]
        whitelist: Vec<String>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Directory in which to search for semantic-release manifest
    #[arg(long, default_value = ".")]
    directory: PathBuf,

    /// Edit file in-place
    #[arg(long, action)]
    in_place: bool,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

fn find_semantic_release_config(directory: &Path) -> Result<PathBuf> {
    Ok(find_semantic_release_configuration(&directory)?.ok_or_else(
        || -> Box<dyn std::error::Error> {
            format!(
                "unable to find semantic-release configuration in {:?}",
                &directory,
            )
            .into()
        },
    )?)
}

fn is_unsupported_file_extension(config: &Path) -> bool {
    match config.extension() {
        Some(extension) => {
            let extension = extension.to_string_lossy();
            !SUPPORTED_FILE_TYPES
                .iter()
                .any(|supported_extension| &extension.as_ref() == supported_extension)
        }
        None => false,
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();
    let config = find_semantic_release_config(&cli.directory)?;

    if is_unsupported_file_extension(&config) {
        eprintln!(
            "Error: unsupported file extension {:?}",
            config.extension().unwrap_or_default()
        );
        eprintln!("Currently configure-semantic-release-manifest only supports the following extensions: {:?}", SUPPORTED_FILE_TYPES);
        return Err("unsupported file extension".into());
    }

    let mut configuration = SemanticReleaseConfiguration::read_from_file(&config)?;

    match cli.subcommand {
        Subcommand::Whitelist {
            whitelist: raw_whitelist,
        } => {
            let whitelist: HashSet<String> = raw_whitelist
                .into_iter()
                .flat_map(|s| {
                    s.split_whitespace()
                        .map(|s| s.to_owned())
                        .collect::<Vec<_>>()
                })
                .collect();

            configuration.apply_whitelist(whitelist);
        }
    }

    match cli.in_place {
        true => configuration.write_if_modified(WriteTo::InPlace)?,
        false => configuration.write_if_modified(WriteTo::Stdout)?,
    };

    Ok(())
}
