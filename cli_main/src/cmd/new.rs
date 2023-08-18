use std::io::Write;
use std::path::PathBuf;

use ddl_core::interface::{self as ddl, new_item};
use ddl_core::item::{ItemTime, ItemUnit};
use ddl_core::{errors::DDLError as ddl_err, item::ItemImportance};

use crate::utils::colorize::{blue, cyan, green, red};

pub fn entry(path: &PathBuf) -> Result<(), ddl_err> {
    let mut entry_key = String::new();

    print!("Give me the entry ");
    blue("key".to_string())?;
    print!(": ");
    std::io::stdout().flush()?;

    std::io::stdin().read_line(&mut entry_key)?;
    entry_key = entry_key.trim().to_string();

    ddl::new_entry(path.clone(), entry_key.clone())?;

    print!("Ok, successfully create the entry: ");
    green(entry_key)?;
    print!("\n");

    Ok(())
}

pub fn item(path: &PathBuf) -> Result<(), ddl_err> {
    let mut entry_key = String::new();
    let mut ddl_label = String::new();

    // entry
    cyan("First, give me the entry ".to_string())?;
    blue("key".to_string())?;
    cyan(": ".to_string())?;
    std::io::stdout().flush()?;

    std::io::stdin().read_line(&mut entry_key)?;
    entry_key = entry_key.trim().to_string();
    while !ddl::new_check_entry(path.clone(), entry_key.clone()) {
        red("Sorry, but the entry ".to_string())?;
        green(entry_key.clone())?;
        red(" does not exists.\n".to_string())?;
        cyan("Try again: ".to_string())?;

        std::io::stdout().flush()?;

        entry_key.clear();
        std::io::stdin().read_line(&mut entry_key)?;
        entry_key = entry_key.trim().to_string();
    }

    // label
    cyan("What's the ".to_string())?;
    blue("label".to_string())?;
    cyan(": ".to_string())?;
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut ddl_label)?;
    ddl_label = ddl_label.trim().to_string();

    // ddl time
    cyan("So, the ".to_string())?;
    blue("ddl".to_string())?;
    cyan(" ([year]-[month]-[day] [hour]:[minute]): ".to_string())?;
    std::io::stdout().flush()?;
    let item_time = loop {
        let mut ori_str = String::new();
        std::io::stdin().read_line(&mut ori_str)?;
        match ItemTime::parse(ori_str.trim().to_string() + " +08") {
            Err(_) => {
                red("Sorry, but the format should be like \"2023-12-31 23:59\".\n".to_string())?;
                cyan("Try again: ".to_string())?;
                std::io::stdout().flush()?;
            }
            Ok(v) => {
                break v;
            }
        }
    };

    // importance
    cyan("Now, type the ".to_string())?;
    blue("importance".to_string())?;
    cyan(" (from 0 to 4): ".to_string())?;
    std::io::stdout().flush()?;
    let importance: u8 = loop {
        let mut ori_str = String::new();
        std::io::stdin().read_line(&mut ori_str)?;
        match ori_str.trim().parse() {
            Err(_) => {
                red("Sorry, but the number you typed is invalid.\n".to_string())?;
                blue("Try again: ".to_string())?;
                std::io::stdout().flush()?;
            }
            Ok(v) => {
                if v <= 4 {
                    break v;
                }
                red("Sorry, but the number should be in [0, 1, 2, 3, 4].\n".to_string())?;
                cyan("Try again: ".to_string())?;
                std::io::stdout().flush()?;
            }
        }
    };
    let importance = ItemImportance::from(importance);

    let item = ItemUnit::new(importance, ddl_label, item_time);

    new_item(path.clone(), entry_key, item)?;

    Ok(())
}
