use crate::get_all_files;
use crate::Template;
use include_dir::{include_dir, Dir};
use std::fs;
use std::io;
use std::path::Path;
use uuid::Uuid;

pub struct TypeScriptStarter;

static TS_TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/ts-starter");

impl Template for TypeScriptStarter {
    fn name(&self) -> &'static str {
        "ts-starter"
    }

    fn description(&self) -> &'static str {
        "A Behavior Pack pre-configured with TypeScript"
    }

    fn generate(&self, target_path: &Path, name: &str, description: &str) -> io::Result<()> {
        let bp_uuid = Uuid::new_v4().to_string();
        let rp_uuid = Uuid::new_v4().to_string();
        let script_uuid = Uuid::new_v4().to_string();

        for entry in get_all_files(&TS_TEMPLATE_DIR) {
            let src_path = entry.path();
            let mut dest_path = target_path.join(name).join(src_path);

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let extension = src_path
                .extension()
                .and_then(|os| os.to_str())
                .unwrap_or("");

            if extension == "tmpl" {
                let mut contents = String::from(entry.contents_utf8().unwrap());

                contents = contents.replace("{{name}}", name);
                contents = contents.replace("{{description}}", description);
                contents = contents.replace("{{bp_uuid}}", bp_uuid.as_str());
                contents = contents.replace("{{rp_uuid}}", rp_uuid.as_str());
                contents = contents.replace("{{script_uuid}}", script_uuid.as_str());
                contents = contents.replace("{{random_uuid}}", &Uuid::new_v4().to_string());
                contents = contents.replace("{{latest_scriptapi_version}}", "2.7.0");

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

    #[test]
    fn get_flat_dir() {
        let files = get_all_files(&TS_TEMPLATE_DIR);
        assert!(!files.is_empty(), "Directory should not be empty");
    }
}
