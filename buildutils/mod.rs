use std::path::Path;
use std::fs;
use std::time::SystemTime;
use std::env;

pub mod sass;
pub mod tailwinds;

use lazy_static::lazy_static;

lazy_static! {
    static ref BIN_TIMESTAMP: Option<SystemTime>  = {
        let out_dir_string = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
        let out_dir = Path::new(&out_dir_string);
        let build_dir = out_dir.parent().unwrap().parent().unwrap().parent().unwrap();
        let filename = env::var("CARGO_PKG_NAME").expect("Failed to get CARGO_BIN_NAME");
        let mut bin_path = build_dir.join(filename);
        if env::var("CARGO_CFG_WINDOWS").is_ok() {
            bin_path.set_extension("exe");
        }
        fs::metadata(&bin_path).ok().map(|m| m.modified().expect("Failed to get modified time"))
    };
}

pub fn needs_rebuild(src: &Path, dest: &Path) -> bool {
    if !dest.try_exists().expect("Failed to check if file exists") {
        return true;
    }
    let src_metadata = fs::metadata(src).unwrap();
    let dest_metadata = fs::metadata(dest).unwrap();
    let src_newer_than_dest = src_metadata.modified().unwrap() > dest_metadata.modified().unwrap();
    if let Some(bin_stamp) = BIN_TIMESTAMP.as_ref() {
        let dest_newer_than_bin = dest_metadata.modified().unwrap() > *bin_stamp;
        return src_newer_than_dest || dest_newer_than_bin;
    }
    src_newer_than_dest
}