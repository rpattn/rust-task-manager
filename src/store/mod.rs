use std::{fs, io, path::{PathBuf}};

pub fn save(filepath: &PathBuf, text: &str) -> Result<(), io::Error> {
    // create parent dir if filename includes folders
    let parent = filepath.parent();
    if let Some(dir) = parent {
        fs::create_dir_all(dir)?;
    }

    fs::write(filepath, text)?;
    Ok(())
}

pub fn load(filepath: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(filepath)
}
