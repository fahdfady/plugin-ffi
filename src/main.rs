use std::fs;

use libloading::Library;

struct PluginInfo {
    name: *const i8,
    author: *const i8,
    version: i32,
}

struct Plugin {
    lib: Library,
    info: PluginInfo,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("./plugins")? {
        let path = entry?.path();
        let ext = path.extension().unwrap();
        if ext == "so" {
            println!("found lib: {}", path.display());
        }
    }

    println!("hello world");

    Ok(())
}
