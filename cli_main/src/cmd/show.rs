use std::path::PathBuf;

use ddl_core::errors::DDLError as ddl_err;
use ddl_core::interface as ddl;

fn disp(item_disp: ddl::ItemUnitDisp) {
    println!("---- {} / {} ----", item_disp.label, item_disp.importance);
    println!(
        "{}DDL: {} / Rest time: {}",
        if item_disp.over { "[Over!] " } else { "" },
        item_disp.ddl,
        item_disp.rest
    );
    let process_block: usize = unsafe { (0.2 * item_disp.percent).to_int_unchecked() };
    print!("|");
    for _ in 0..process_block {
        print!("=");
    }
    print!(">");
    for _ in process_block..20 {
        print!("-");
    }
    println!("|{:7.2}%", item_disp.percent);
    println!();
}

pub fn select_all(path: &PathBuf) -> Result<(), ddl_err> {
    let vec = ddl::get_item_whole(path.clone())?;
    for sub in vec {
        println!("\n[==== {} ====]", sub.0);
        for item in sub.1 {
            disp(item);
        }
    }
    Ok(())
}

pub fn select_by_key(path: &PathBuf, key: String) -> Result<(), ddl_err> {
    let vec = ddl::get_item_list_by_key(path.clone(), &key)?;
    println!("\n[==== {} ====]", key);
    for item in vec {
        disp(item);
    }
    Ok(())
}
