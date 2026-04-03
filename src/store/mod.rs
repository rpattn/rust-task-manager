use std::{fs, io, path::Path};

pub fn save(filename: &str, text: &str) -> Result<(), io::Error> {
    // create parent dir if filename includes folders
    let parent = Path::new(filename).parent();
    if let Some(dir) = parent {
        fs::create_dir_all(dir)?;
    }

    fs::write(filename, text)?;
    Ok(())
}

pub fn load(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
