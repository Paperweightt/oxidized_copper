use std::ffi::OsStr;
use std::fs::{self};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn validate_pack(dir: &Path) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() && path.file_name().unwrap() == OsStr::new("manifest.json") {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn find_packs(dir: Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {
    let mut packs: Vec<PathBuf> = Vec::new();

    for path in dir {
        let walker = WalkDir::new(path).into_iter().filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();

            if validate_pack(path)? {
                packs.push(path.to_path_buf());
            }
        }
    }

    return Ok(packs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_validator() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../test/example_pack/behavior_packs/pack0/");

        assert!(validate_pack(new_path.as_path()).unwrap());
    }
    #[test]
    fn test_find_packs() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let paths: Vec<PathBuf> = vec![
            path.join("../../test/example_pack/"),
            path.join("../../test/example_pack/resource_packs/pack0/"),
        ];

        let result = find_packs(paths);

        assert_eq!(result.unwrap().len(), 3);
    }
}
