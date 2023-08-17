use std::path::PathBuf;

use ddl_core::errors::{DDLError as ddl_err, TerminalError};
use ddl_core::interface as ddl;

use crate::utils::{
    colorize::{blue, cyan, gray, green, red, Auto},
    wordwrap::{entry_title, process_bar, time_info},
};

macro_rules! colorize_expand {
    ($( $n:tt ),* ; $str_tup:expr => $color_tup:expr) => {
        $($color_tup.$n($str_tup.$n)?;)*
    };
}

macro_rules! colorize {
    (3; $str_tup:expr => $color_tup:expr) => {
        colorize_expand!(0, 1, 2 ; $str_tup => $color_tup)
    };
    (6; $str_tup:expr => $color_tup:expr) => {
        colorize_expand!(0, 1, 2, 3, 4, 5 ; $str_tup => $color_tup)
    };
}

fn disp(item_disp: ddl::ItemUnitDisp, width: u16) -> Result<(), ddl_err> {
    // Title
    cyan("> ".to_string())?;
    (if item_disp.over { gray } else { blue })(format!("{}\n", item_disp.label))?;

    // [Over!] 2022-01-01 00:00     Huge Importance     0 day 23 hour 59 minute
    let info = time_info(
        width,
        item_disp.over,
        item_disp.ddl,
        item_disp.importance,
        item_disp.rest,
    );
    colorize!(6; info => (
        gray,
        if item_disp.over { gray } else if item_disp.percent > 80.0 { red } else { green },
        gray,
        if item_disp.over { gray } else { Auto::importance },
        gray,
        if item_disp.over { gray } else if item_disp.percent > 80.0 { red } else { green },
    ));

    // Process bar  [ ==== > ---- ] ***.**%
    let bar = process_bar(width, item_disp.percent);
    colorize!(6; bar =>
    (
        cyan,
        if item_disp.over { gray } else if item_disp.percent > 80.0 { red } else { green },
        cyan,
        gray,
        cyan,
        if item_disp.over { gray } else if item_disp.percent > 80.0 { red } else { green },
    ));
    print!("\n\n");
    Ok(())
}

pub fn select_all(path: &PathBuf) -> Result<(), ddl_err> {
    let vec = ddl::get_item_whole(path.clone())?;
    let w = crate::utils::wordwrap::term_width().ok_or(ddl_err::TerminalError(TerminalError {}))?;
    for sub in vec {
        let title = entry_title(w, sub.0);
        colorize!(3; title => (cyan, blue, cyan));
        for item in sub.1 {
            disp(item, w)?;
        }
    }
    Ok(())
}

pub fn select_by_key(path: &PathBuf, key: String) -> Result<(), ddl_err> {
    let vec = ddl::get_item_list_by_key(path.clone(), &key)?;
    let w = crate::utils::wordwrap::term_width().ok_or(ddl_err::TerminalError(TerminalError {}))?;
    let title = entry_title(w, key);
    colorize!(3; title => (cyan, blue, cyan));
    for item in vec {
        disp(item, w)?;
    }
    Ok(())
}
