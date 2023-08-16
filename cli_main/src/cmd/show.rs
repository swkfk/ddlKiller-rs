use std::path::PathBuf;

use ddl_core::errors::DDLError as ddl_err;
use ddl_core::interface as ddl;

use crate::utils::colorize::{cyan, gray, green, red, Auto};

fn disp(item_disp: ddl::ItemUnitDisp) -> Result<(), ddl_err> {
    if item_disp.over {
        gray(format!(
            "---- {} / {} ----\n[Over!] DDL: {} / Rest time: {}",
            item_disp.label, item_disp.importance, item_disp.ddl, item_disp.rest
        ))?;
    } else {
        cyan(format!("---- {} / ", item_disp.label))?;
        Auto::importance(item_disp.importance)?;
        cyan(" ----\nDDL: ".to_string())?;
        if item_disp.percent > 80.0 {
            red(item_disp.ddl)?;
        } else {
            green(item_disp.ddl)?;
        }
        cyan(" / Rest time: ".to_string())?;
        if item_disp.percent > 80.0 {
            red(item_disp.rest)?;
        } else {
            green(item_disp.rest)?;
        }
        // Need a '\n'!
    }
    let process_block: usize = unsafe { (0.2 * item_disp.percent).to_int_unchecked() };
    cyan("\n[".to_string())?;
    for _ in 0..process_block {
        if item_disp.over {
            gray("=".to_string())?;
        } else if item_disp.percent > 80.0 {
            red("=".to_string())?;
        } else {
            green("=".to_string())?;
        }
    }
    red(">".to_string())?;
    for _ in process_block..20 {
        gray("-".to_string())?;
    }
    cyan("]".to_string())?;
    if item_disp.percent > 80.0 && !item_disp.over {
        red(format!("{:7.2}%", item_disp.percent))?;
    } else if !item_disp.over {
        green(format!("{:7.2}%", item_disp.percent))?;
    } else {
        gray(format!("{:7.2}%", item_disp.percent))?;
    }
    print!("\n\n");
    Ok(())
}

pub fn select_all(path: &PathBuf) -> Result<(), ddl_err> {
    let vec = ddl::get_item_whole(path.clone())?;
    for sub in vec {
        println!("\n[==== {} ====]", sub.0);
        for item in sub.1 {
            disp(item)?;
        }
    }
    Ok(())
}

pub fn select_by_key(path: &PathBuf, key: String) -> Result<(), ddl_err> {
    let vec = ddl::get_item_list_by_key(path.clone(), &key)?;
    println!("\n[==== {} ====]", key);
    for item in vec {
        disp(item)?;
    }
    Ok(())
}
