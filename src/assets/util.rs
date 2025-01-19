use include_directory::{include_directory, Dir, File};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_directory!("assets");

pub fn read_any_file<'a>(name: &'a str, path: &'a str) -> File<'a> {
    let path = Path::new(path).join(name);
    let file = PROJECT_DIR.get_file(path.clone()).unwrap_or_else(|| {
        panic!(
            "could not find icon with this name: {}",
            path.to_str().unwrap()
        )
    });
    file.clone()
}
