use std::{
    collections::HashSet,
    ffi::OsString,
    fmt::Display,
    path::{Path, PathBuf},
};

use clap::Parser;
use find_semantic_release_config::find_semantic_release_configuration;

use configure_semantic_release_assets::{SemanticReleaseConfiguration, WriteTo};

mod cli;
mod little_anyhow;

use cli::Cli;

const SUPPORTED_FILE_TYPES: &[&str] = &["json"];

#[derive(Debug)]
#[non_exhaustive]
struct Error {
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::FindConfiguration(_) => {
                write!(f, "unable to find semantic-release configuration")
            }
            ErrorKind::MissingConfiguration { directory } => write!(
                f,
                "directory does not contain semantic-release configuration: {:?}",
                directory
            ),
            ErrorKind::UnsupportedFileExtension { extension } => {
                writeln!(
                    f,
                    "unsupported file extension {:?}",
                    extension.clone().unwrap_or_default()
                )?;
                write!(f, "Currently configure-semantic-release-manifest only supports the following extensions: {:?}", SUPPORTED_FILE_TYPES)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::FindConfiguration(err) => Some(err),
            ErrorKind::MissingConfiguration { directory: _ } => None,
            ErrorKind::UnsupportedFileExtension { extension: _ } => None,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    #[non_exhaustive]
    FindConfiguration(find_semantic_release_config::Error),
    #[non_exhaustive]
    MissingConfiguration { directory: PathBuf },
    #[non_exhaustive]
    UnsupportedFileExtension { extension: Option<OsString> },
}

impl From<ErrorKind> for little_anyhow::Error {
    fn from(kind: ErrorKind) -> Self {
        let error = Error { kind };
        error.into()
    }
}

fn find_semantic_release_config(directory: &Path) -> Result<PathBuf, ErrorKind> {
    Ok(find_semantic_release_configuration(&directory)
        .map_err(ErrorKind::FindConfiguration)?
        .ok_or_else(|| ErrorKind::MissingConfiguration {
            directory: directory.to_owned(),
        })?)
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

fn main() -> Result<(), little_anyhow::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();
    let config = find_semantic_release_config(&cli.directory)?;

    if is_unsupported_file_extension(&config) {
        return Err(ErrorKind::UnsupportedFileExtension {
            extension: config.extension().map(ToOwned::to_owned),
        })?;
    }

    let mut configuration = SemanticReleaseConfiguration::read_from_file(&config)?;

    match cli.subcommand {
        cli::Subcommand::Whitelist {
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
