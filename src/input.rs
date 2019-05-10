use std::fs;
use std::path;

use crate::options::InputOptions;

fn iterate_dir<F>(config: &InputOptions, path: &path::Path, callback: &mut F)
    where F: FnMut(&path::Path)
{
    for dir in &config.directories {
        for entry in fs::read_dir(dir)
            .expect(&format!("{} is not a valid directory", path.display()))
        {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                iterate_dir(config, &entry.path(), callback)
            } else {
                callback(&entry.path())
            }
        }
    }
}

pub fn get_input_files(config: &InputOptions) -> Vec<path::PathBuf> {
    let mut paths = Vec::new();
    for directory in &config.directories {
        iterate_dir(config, directory, &mut |path| {
            paths.push(path.to_path_buf());
        });
    }
    paths
}

pub fn load_image(path: &path::Path) -> image::DynamicImage {
    image::open(path).expect(&format!("Unable to load {}", path.display()))
}