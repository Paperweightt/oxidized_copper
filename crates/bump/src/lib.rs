use clap::ValueEnum;
use std::cmp;
use std::path::PathBuf;

use mcbe_core::manifest::Manifest;

pub struct BumpArgs {
    pub paths: Vec<PathBuf>,
    pub r#type: BumpVersion,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BumpVersion {
    Minor,
    Major,
    Fix,
}

pub fn bump_pack(args: BumpArgs) {
    let mut manifests: Vec<Manifest> = Vec::new();
    let mut min_version = [0, 0, 0];

    for path in args.paths {
        let manifest = match Manifest::new(path) {
            Ok(manifest) => manifest,
            Err(error) => {
                eprintln!("[mcbe_cli] Problem parsing the manifest: {error}");
                continue;
            }
        };

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
        manifest.set_version(min_version);

        if let Err(error) = manifest.save() {
            println!("[mcbe_cli] Problem saving the manifest: {error}");
        } else {
            eprintln!("[mcbe_cli] Successfully set manifest to {min_version:?}")
        }
    }
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
}
