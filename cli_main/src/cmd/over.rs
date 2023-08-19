use std::path::PathBuf;

use ddl_core::errors::DDLError as ddl_err;
use ddl_core::interface as ddl;

use crate::utils::colorize::cyan;

pub fn item(path: &PathBuf, entry: String, key: usize) -> Result<(), ddl_err> {
    match ddl::over_item(path, entry, key) {
        Ok(b) => {
            if b {
                cyan("Congratulations!\n".to_string())?;
            } else {
                cyan("But this ddl seemed to have been done.\n".to_string())?;
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn entry(path: &PathBuf, entry: String) -> Result<(), ddl_err> {
    match ddl::over_entry(path, entry) {
        Ok(b) => {
            if b {
                cyan("Congratulations!\n".to_string())?;
            } else {
                cyan("But this ddl seemed to have been done.\n".to_string())?;
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
