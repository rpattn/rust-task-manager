use std::{fs, io, path::Path};

pub fn save(filename: &str, text: &str) -> Result<(), io::Error> {

    // create parent dir if filename includes folders, unwrap any error
    let parent = Path::new(filename).parent();
    if let Some(dir) = parent {
        fs::create_dir_all(dir).unwrap()
    }

    fs::write(filename, text)
}

pub fn load(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
