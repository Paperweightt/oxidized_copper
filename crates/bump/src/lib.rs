#![allow(dead_code)]

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{cmp, fs, io};

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestJSON {
    format_version: u32,
    header: ManifestHeader,
    modules: Vec<ManifestModule>,
    dependencies: Vec<ManifestDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ManifestHeader {
    name: String,
    version: [i32; 3],
    description: String,
    uuid: String,
    min_engine_version: [i32; 3],
}

#[derive(Debug, Deserialize, Serialize)]
struct ManifestModule {
    version: Version,
    r#type: String,
    uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ManifestDependency {
    version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Version {
    String(String),
    Array([i32; 3]),
}

pub struct Manifest {
    pub json: ManifestJSON,
    pub path: PathBuf,
}

impl Manifest {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let data = fs::read_to_string(&path)?;

        let manifest: ManifestJSON = serde_json::from_str(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Manifest {
            json: manifest,
            path,
        })
    }

    pub fn save(&self) -> io::Result<()> {
        let modified_json = serde_json::to_string_pretty(&self.json)?;
        fs::write(&self.path, modified_json)?;
        Ok(())
    }

    pub fn get_version(&self) -> [i32; 3] {
        let mut min_version: [i32; 3] = self.json.header.version;

        for module in &self.json.modules {
            if let Version::Array(version) = module.version {
                min_version[0] = cmp::max(version[0], min_version[0]);
                min_version[1] = cmp::max(version[1], min_version[1]);
                min_version[2] = cmp::max(version[2], min_version[2]);
            }
        }

        for module in &self.json.dependencies {
            if let Version::Array(version) = module.version {
                min_version[0] = cmp::max(version[0], min_version[0]);
                min_version[1] = cmp::max(version[1], min_version[1]);
                min_version[2] = cmp::max(version[2], min_version[2]);
            }
        }

        min_version
    }

    pub fn set_version(&mut self, new_version: [i32; 3]) {
        self.json.header.version = new_version;

        for module in &mut self.json.modules {
            if let Version::Array(ref mut version) = module.version {
                *version = new_version;
            }
        }

        for module in &mut self.json.dependencies {
            if let Version::Array(ref mut version) = module.version {
                *version = new_version;
            }
        }
    }
}

pub struct BumpArgs {
    pub path: PathBuf,
    pub r#type: BumpVersion,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BumpVersion {
    Minor,
    Major,
    Fix,
}

pub fn bump_pack(args: BumpArgs) -> Option<()> {
    let pack_paths = mcbe_core::find_packs(args.path).ok()?;
    let mut manifests: Vec<Manifest> = Vec::new();
    let mut min_version = [0, 0, 0];

    for mut path in pack_paths {
        path.push("manifest.json");
        let manifest = Manifest::new(path).ok()?;
        let version = manifest.get_version();

        min_version[0] = cmp::max(version[0], min_version[0]);
        min_version[1] = cmp::max(version[1], min_version[1]);
        min_version[2] = cmp::max(version[2], min_version[2]);

        manifests.push(manifest);
    }

    match args.r#type {
        BumpVersion::Major => min_version[0] += 1,
        BumpVersion::Minor => min_version[1] += 1,
        BumpVersion::Fix => min_version[2] += 1,
    }

    for mut manifest in manifests {
        println!("{:?}", min_version);
        manifest.set_version(min_version);
        manifest.save().ok()?;
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_manifest() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../test/example_pack/behavior_packs/pack0/manifest.json");
        let manifest = Manifest::new(new_path).unwrap();

        assert_eq!(manifest.json.header.name, "Vectora Editor");
    }
    #[test]
    fn set_and_get_pack_version() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../test/example_pack/behavior_packs/pack0/manifest.json");
        let mut manifest = Manifest::new(new_path).unwrap();

        manifest.set_version([5, 5, 5]);

        assert_eq!(manifest.get_version(), [5, 5, 5]);
    }

    #[test]
    fn bump_pack_version() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../test/example_pack/behavior_packs/pack0");

        bump_pack(BumpArgs {
            path: new_path,
            r#type: BumpVersion::Fix,
        });
    }
}
