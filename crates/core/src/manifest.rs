#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use std::{cmp, fs, io};

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestJSON {
    pub format_version: u32,
    pub header: ManifestHeader,
    pub modules: Vec<ManifestModule>,
    pub dependencies: Vec<ManifestDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestHeader {
    pub name: String,
    pub version: [i32; 3],
    pub description: String,
    pub uuid: String,
    pub min_engine_version: [i32; 3],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestModule {
    pub version: Version,
    pub r#type: String,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestDependency {
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Version {
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
