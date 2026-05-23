use clap::ValueEnum;
use include_dir::{Dir, File};
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

mod ts_starter;

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

pub trait Template {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn generate(&self, target_path: &Path, name: &str, description: &str) -> io::Result<()>;
}

pub struct TemplateRegistry {
    templates: HashMap<&'static str, Box<dyn Template>>,
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        registry.register(ts_starter::TypeScriptStarter);

        registry
    }

    fn register<T: Template + 'static>(&mut self, template: T) {
        self.templates.insert(template.name(), Box::new(template));
    }

    pub fn list(&self) -> Vec<(&'static str, &'static str)> {
        self.templates
            .values()
            .map(|t| (t.name(), t.description()))
            .collect()
    }

    pub fn instantiate(
        &self,
        template_name: &str,
        target_path: &Path,
        name: &str,
        description: &str,
    ) -> io::Result<()> {
        if let Some(template) = self.templates.get(template_name) {
            template.generate(target_path, name, description)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("[mcbe_cli] Template '{}' does not exist.", template_name),
            ))
        }
    }
}

pub fn handle_init_command(
    template_name: &str,
    output_dir: PathBuf,
    name: &str,
    description: &str,
) {
    let registry = TemplateRegistry::new();

    println!("[mcbe_cli] Initializing template: {}...", template_name);

    match registry.instantiate(template_name, &output_dir, name, description) {
        Ok(_) => println!("[mcbe_cli] Successfully generated project template"),
        Err(error) => eprintln!("[mcbe_cli] Error generating template: {error}"),
    }
}

pub fn get_all_files(dir: &'static Dir<'static>) -> Vec<&'static File<'static>> {
    let mut list: Vec<&'static File<'static>> = Vec::new();
    let mut dirs: Vec<&'static Dir<'static>> = Vec::from([dir]);

    while !dirs.is_empty() {
        let search_dir = match dirs.pop() {
            Some(d) => d,
            None => continue,
        };

        for dir in search_dir.dirs() {
            dirs.push(dir);
        }

        for file in search_dir.files() {
            list.push(file);
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let new_path = path.join("../../test");

        let registry = TemplateRegistry::new();
        match registry.instantiate(
            "ts-starter",
            &new_path,
            "very cool name",
            "long cool description",
        ) {
            Ok(_) => println!("[mcbe_cli] Successfully generated project template"),
            Err(error) => eprintln!("[mcbe_cli] Error generating template: {error}"),
        }
    }
}
