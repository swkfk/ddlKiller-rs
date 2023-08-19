use std::path::PathBuf;

use ddl_core::errors::DDLError as ddl_err;
use ddl_core::interface as ddl;

use crate::utils::{
    colorize::{blue, cyan, gray, green},
    tablelize::{entry_head, item_list},
};

pub fn select_all(path: &PathBuf) -> Result<(), ddl_err> {
    let list = ddl::list_entry(path.clone())?;
    for entry in list {
        if !entry.2 {
            continue;
        }
        print!("\n");
        let node = entry_head(entry.0, entry.1.clone(), (&cyan, &blue))?;
        select_by_key(path, entry.1.clone(), node)?;
    }
    Ok(())
}

pub fn select_by_key(path: &PathBuf, key: String, node: (u16, u16)) -> Result<(), ddl_err> {
    let vec = ddl::list_item_by_key(path.clone(), &key)?;
    item_list(vec, (&cyan, &blue, &green, &gray), node)?;
    Ok(())
}
