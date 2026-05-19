#![expect(clippy::print_stdout)]
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};
use oxc_data_structures::code_buffer::IndentChar;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{HelperLoaderMode, TransformOptions, Transformer};
use std::path::{Path, PathBuf};

use std::fs;

pub fn transpile(source: &Path, destination: &Path, source_map: Option<PathBuf>) {
    let source_type = SourceType::from_path(source).unwrap();
    let source_text = std::fs::read_to_string(source)
        .unwrap_or_else(|err| panic!("{} not found.\n{}", source.display(), err,));
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
        println!("Parser Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let mut program = ret.program;

    let ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .with_enum_eval(true)
        .build(&program);

    if !ret.errors.is_empty() {
        println!("Semantic Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let scoping = ret.semantic.into_scoping();
    let mut transform_options = TransformOptions::from_target("es2022").unwrap();

    transform_options.helper_loader.mode = HelperLoaderMode::External;

    let ret = Transformer::new(&allocator, source, &transform_options)
        .build_with_scoping(scoping, &mut program);

    if !ret.errors.is_empty() {
        println!("Transformer Errors:");
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }

    let mut options = CodegenOptions {
        indent_char: IndentChar::Space,
        indent_width: 2,
        ..CodegenOptions::default()
    };

    if let Some(source_map) = &source_map {
        options.source_map_path = Some(source_map.to_path_buf());
    }

    let CodegenReturn { code, map, .. } = Codegen::new().with_options(options).build(&program);

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| println!("{err}"));
    }

    if let Some(map) = map {
        let sourcemap_json = map.to_json_string();
        let path = source_map.unwrap();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|err| println!("{err}"));
        }

        fs::write(path, sourcemap_json).unwrap_or_else(|err| println!("{err}"));
    }

    fs::write(destination, code).unwrap_or_else(|err| println!("{err}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpile_test() {
        transpile(
            Path::new("./scripts/test.ts"),
            Path::new("./mojang/test.js"),
            Some(PathBuf::from("./dist/debug/main.js.map")),
        )
    }
}
