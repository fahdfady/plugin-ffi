use std::fs;

use libloading::{Library, Symbol};

struct PluginInfo {
    name: *const i8,
    author: *const i8,
    version: i32,
}

struct Plugin {
    //lib: Library,
    info: PluginInfo,
    process: unsafe extern "C" fn(*mut f32, i32),
    cleanup: unsafe extern "C" fn(),
}

impl Plugin {
    fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let get_info: Symbol<unsafe extern "C" fn() -> PluginInfo> =
                lib.get(b"get_plugin_info")?;
            let info = get_info();
            let process: Symbol<unsafe extern "C" fn(*mut f32, i32)> = lib.get(b"process_audio")?;
            let cleanup: Symbol<unsafe extern "C" fn()> = lib.get(b"plugin_cleanup")?;

            Ok(Self {
                // lib,
                info,
                process: *process,
                cleanup: *cleanup,
            })
        }
    }
    pub fn process(&self, buffer: &mut [f32]) {
        unsafe { (self.process)(buffer.as_mut_ptr(), buffer.len() as i32) }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut audio_buffer = vec![0.2, 0.3, -0.5, 0.1];
    for entry in fs::read_dir("./plugins")? {
        let path = entry?.path();
        let ext = path.extension().unwrap();
        if ext == "so" {
            println!("found lib: {}", path.display());
            let plugin = Plugin::load(path.to_str().unwrap())?;
            plugin.process(&mut audio_buffer);
        }
    }

    println!("hello world");

    Ok(())
}
