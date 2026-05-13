use std::ffi::OsStr;
use std::fs::{self};
use std::io;
use std::path::Path;

pub fn validate_pack(dir: &Path) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() && path.file_name() == Some(OsStr::new("manifest.json")) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = Path::new("../../test/example_pack/behavior_packs/pack0/");
        assert!(validate_pack(path).unwrap());
    }
}
