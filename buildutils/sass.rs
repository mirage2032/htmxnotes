use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use super::needs_rebuild;

fn is_stylesheet(path: &Path) -> bool {
    let suffixes = ["scss", "sass", "css"];
    if let Some(extension) = path.extension() {
        return suffixes
            .iter()
            .any(|suffix| extension.to_str().unwrap() == *suffix);
    }
    false
}

pub fn compile(src_dir: &Path, dest_dir: &Path) {
    println!("cargo:rerun-if-changed={}", src_dir.display());
    // println!("Removing old css directory: {}", dest_dir.display());
    // if dest_dir.exists() {
    //     std::fs::remove_dir_all(dest_dir).expect("Failed to remove directory");
    // }
    println!("Compiling all style sheets from: {}", src_dir.display());
    let walker = WalkDir::new(src_dir).min_depth(1);
    for entry in walker {
        let entry = entry.expect("Failed to read entry");
        let source_path = entry.path();
        if source_path.is_file() && is_stylesheet(source_path) {
            let mut destination_path = PathBuf::from(dest_dir);
            let relative_path = source_path
                .strip_prefix(src_dir)
                .expect("Failed to strip prefix");
            destination_path.push(relative_path);
            destination_path.set_extension("css");
            if !needs_rebuild(source_path, &destination_path) {
                println!("Skipping up-to-date: {}", source_path.display());
                continue;
            }
            println!("Compiling: {}", source_path.display());
            let path_str = source_path
                .to_str()
                .expect("Failed to convert path to string");
            let css =
                grass::from_path(path_str, &grass::Options::default()).expect("Failed to compile");
            if css.is_empty() {
                println!("Skipping as it did not result in any css");
                continue;
            }
            std::fs::create_dir_all(destination_path.parent().expect("Failed to get parent"))
                .expect("Failed to create directory");
            std::fs::write(destination_path, css).expect("Failed to write file");
        }
    }
    println!("Compilation completed successfully");
}
