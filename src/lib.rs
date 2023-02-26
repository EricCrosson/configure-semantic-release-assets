#![forbid(unsafe_code)]
#![deny(warnings)]

use std::{
    collections::HashSet,
    io::{self, BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};
use std::{fs::File, io::Read, path::Path};

use indexmap::{map::Entry, IndexMap};
use log::debug;

mod error;

use crate::error::Error;

#[derive(Debug)]
pub enum WriteTo {
    Stdout,
    InPlace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModifiedFlag {
    Unmodified,
    Modified,
}

#[derive(Debug)]
pub struct SemanticReleaseManifest {
    inner: IndexMap<String, serde_json::Value>,
}

impl FromStr for SemanticReleaseManifest {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner: serde_json::from_str(s)?,
        })
    }
}

pub struct SemanticReleaseConfiguration {
    manifest: SemanticReleaseManifest,
    manifest_path: PathBuf,
    dirty: ModifiedFlag,
}

fn plugin_name(plugin: &serde_json::Value) -> Option<&str> {
    match plugin {
        serde_json::Value::String(name) => Some(name.as_str()),
        serde_json::Value::Array(array) => array.get(0).and_then(|value| value.as_str()),
        _ => None,
    }
}

fn plugin_configuration(
    plugin: &mut serde_json::Value,
) -> Option<&mut serde_json::Map<String, serde_json::Value>> {
    match plugin {
        serde_json::Value::Array(array) => array.get_mut(1).and_then(|value| value.as_object_mut()),
        _ => None,
    }
}

impl SemanticReleaseManifest {
    pub fn apply_whitelist(&mut self, whitelist: HashSet<String>) -> ModifiedFlag {
        let mut dirty = ModifiedFlag::Unmodified;

        if let Entry::Occupied(mut entry) = self.inner.entry("plugins".to_owned()) {
            if let Some(plugins) = entry.get_mut().as_array_mut() {
                for plugin in plugins {
                    if plugin_name(plugin) != Some("@semantic-release/github") {
                        continue;
                    }

                    if let Some(assets) = plugin_configuration(plugin)
                        .and_then(|settings| settings.get_mut("assets"))
                        .and_then(|assets| assets.as_array_mut())
                    {
                        assets.retain(|asset| {
                            let label = asset
                                .as_object()
                                .and_then(|asset| asset.get("label"))
                                .and_then(|label| label.as_str());
                            match label {
                                Some(label) => {
                                    let keep = whitelist.contains(label);
                                    if !keep {
                                        dirty = ModifiedFlag::Modified;
                                    }
                                    keep
                                }
                                // Not sure what this is, so pass it through unchanged
                                None => true,
                            }
                        });
                    };
                }
            }
        };

        dirty
    }
}

impl std::fmt::Display for SemanticReleaseManifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.inner).unwrap())
    }
}

impl SemanticReleaseConfiguration {
    // FIXME: use find-semantic-release-manifest.
    // For now, assume the semantic-release configuration is a .releaserc.json and document the limitation
    pub fn read_from_file(semantic_release_manifest_path: &Path) -> Result<Self, Error> {
        debug!(
            "Reading semantic-release configuration from file {:?}",
            semantic_release_manifest_path
        );

        if !semantic_release_manifest_path.exists() {
            return Err(Error::configuration_file_not_found_error(
                semantic_release_manifest_path,
            ));
        }

        // Reading a file into a string before invoking Serde is faster than
        // invoking Serde from a BufReader, see
        // https://github.com/serde-rs/json/issues/160
        let mut string = String::new();
        File::open(semantic_release_manifest_path)
            .map_err(|err| Error::file_open_error(err, semantic_release_manifest_path))?
            .read_to_string(&mut string)
            .map_err(|err| Error::file_read_error(err, semantic_release_manifest_path))?;

        Ok(Self {
            manifest: SemanticReleaseManifest::from_str(&string)
                .map_err(|err| Error::file_parse_error(err, semantic_release_manifest_path))?,
            manifest_path: semantic_release_manifest_path.to_owned(),
            dirty: ModifiedFlag::Unmodified,
        })
    }

    fn write(&mut self, mut w: impl Write) -> Result<(), Error> {
        debug!(
            "Writing semantic-release configuration to file {:?}",
            self.manifest_path
        );
        serde_json::to_writer_pretty(&mut w, &self.manifest.inner)
            .map_err(Error::file_serialize_error)?;
        w.write_all(b"\n")
            .map_err(|err| Error::file_write_error(err, &self.manifest_path))?;
        w.flush()
            .map_err(|err| Error::file_write_error(err, &self.manifest_path))?;

        Ok(())
    }

    pub fn write_if_modified(&mut self, write_to: WriteTo) -> Result<(), Error> {
        match self.dirty {
            ModifiedFlag::Unmodified => Ok(()),
            ModifiedFlag::Modified => match write_to {
                WriteTo::Stdout => self.write(io::stdout()),
                WriteTo::InPlace => {
                    let file = File::create(&self.manifest_path)
                        .map_err(|err| Error::file_open_error(err, &self.manifest_path))?;
                    self.write(BufWriter::new(file))
                }
            },
        }
    }

    pub fn apply_whitelist(&mut self, to_remove: HashSet<String>) {
        let modified = self.manifest.apply_whitelist(to_remove);
        if modified == ModifiedFlag::Modified {
            self.dirty = ModifiedFlag::Modified;
        }
    }
}
