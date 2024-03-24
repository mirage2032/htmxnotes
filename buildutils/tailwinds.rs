use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use railwind::{CollectionOptions, Source, SourceOptions};
use walkdir::WalkDir;

pub fn compile(templates: &Path, destination: &Path) {
    println!("cargo:rerun-if-changed={}", templates.display());
    println!("Compiling Tailwind for all templates in: {}", templates.display());
    let walker = WalkDir::new(templates).min_depth(1);
    let mut source_paths: Vec<PathBuf> = Vec::new();
    for entry in walker {
        let entry = entry.expect("Failed to read entry");
        let source_path = entry.path();
        if source_path.is_file() && source_path.extension() == Some(OsStr::new("html"))
        {
            source_paths.push(PathBuf::from(source_path));
        }
    }
    let sourceopts = source_paths.iter().map(|p|
        SourceOptions {
            input: p,
            option: CollectionOptions::Html,
        }
    ).collect();

    let destination_str = destination.to_str().expect("Failed to destination path to string");
    let mut warnings = Vec::new();
    railwind::parse_to_file(Source::Files(sourceopts), destination_str, false, &mut warnings);
    for w in &warnings {
        println!("Warning: {}", w);
    }
    if !warnings.is_empty() {
        panic!("Compilation completed with warnings");
    } else {
        println!("Compilation +completed successfully");
    }
}