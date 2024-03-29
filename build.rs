mod buildutils;

use std::path::Path;

fn main() {
    buildutils::sass::compile(Path::new("sass"), Path::new("static/css"));
    buildutils::tailwinds::compile(Path::new("templates"), Path::new("static/css/tailwind.css"));
    println!("cargo:rerun-if-changed=static/css");
}
