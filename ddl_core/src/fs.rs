use std::{
    fs::{self, create_dir, File},
    io::Write,
    path::PathBuf,
};

use crate::{
    entry::EntrySet,
    errors::{DDLError, HomeDirNotAvailable},
};

pub fn ensure_data_dir() -> Result<PathBuf, DDLError> {
    let dir = home::home_dir()
        .ok_or(HomeDirNotAvailable {})?
        .join(".ddl-killer");
    if !dir.exists() {
        create_dir(dir.clone())?;
    }
    Ok(dir)
}

impl EntrySet {
    pub fn read_entry(path: PathBuf) -> Result<EntrySet, DDLError> {
        let path = path.join("entry.toml");
        let content;
        if !path.is_file() {
            let mut file = File::create(path)?;
            content = "entries = []\n".to_string();
            file.write_all(content.as_bytes())?;
        } else {
            content = fs::read_to_string(path)?;
        }
        Ok(EntrySet::deserialize(content)?)
    }

    pub fn write_entry(&self, path: PathBuf) -> Result<(), DDLError> {
        let s = self.serialize()?;
        let path = path.join("entry.toml");
        fs::write(path, s)?;
        Ok(())
    }
}
