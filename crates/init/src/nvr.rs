use crate::get_all_files;
use crate::ts_starter;
use crate::Template;
use include_dir::{include_dir, Dir};
use std::fs;
use std::io;
use std::path::Path;

pub struct Nvr;

static NVR_TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/nvr");

impl Template for Nvr {
    fn name(&self) -> &'static str {
        "nvr"
    }

    fn description(&self) -> &'static str {
        "An Addon pack pre-configured with TypeScript with Neovim-Remote support"
    }

    fn generate(
        &self,
        target_path: &Path,
        name: &str,
        description: &str,
        _ignore_files: Vec<&Path>,
    ) -> io::Result<()> {
        let ignore = vec![Path::new("esbuild.config.mjs"), Path::new("README.md")];

        ts_starter::TypeScriptStarter.generate(target_path, name, description, ignore)?;

        for entry in get_all_files(&NVR_TEMPLATE_DIR) {
            let src_path = entry.path();
            let mut dest_path = target_path.join(src_path);

            let extension = src_path
                .extension()
                .and_then(|os| os.to_str())
                .unwrap_or("");

            if extension == "tmpl" {
                let mut contents = String::from(entry.contents_utf8().unwrap());

                contents = contents.replace("{{name}}", name);
                contents = contents.replace("{{description}}", description);

                dest_path.set_extension("");

                fs::write(dest_path, contents)?;
            } else {
                fs::write(dest_path, entry.contents_utf8().unwrap())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_dir = path.join("../../temp");
        let ignore: Vec<&Path> = Vec::new();
        let description = "hi";

        Nvr.generate(
            &test_dir.clone().join("./nvr_test"),
            "duck",
            description,
            ignore,
        )
        .unwrap();
    }
}
