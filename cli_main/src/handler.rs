use std::process::exit;

use ddl_core::errors::DDLError as ddl_err;

pub fn default_dir(e: ddl_err) -> ! {
    match e {
        ddl_err::HomeDirNotAvailable(_) => eprintln!("Home directory not available!"),
        ddl_err::StdIOError(e) => eprintln!("Cannot create the program's data directory: {}", e),
        _ => eprintln!("Unknown error!"),
    }
    exit(1);
}

pub fn args_parse(e: ddl_err) -> ! {
    match e {
        ddl_err::ArgsParseError(e) => eprintln!("{}", e),
        _ => eprintln!("Unknown error!"),
    }
    exit(1);
}

pub fn cmd_show(e: ddl_err) -> ! {
    match e {
        ddl_err::EntryKeyNotFound(e) => eprintln!("{}", e),
        ddl_err::StdIOError(e) => eprintln!("Std IO error: {}", e),
        ddl_err::TerminalError(e) => eprintln!("{}", e),
        _ => eprintln!("Unknown error!"),
    }
    exit(1);
}

pub fn cmd_list(e: ddl_err) -> ! {
    match e {
        ddl_err::EntryKeyNotFound(e) => eprintln!("{}", e),
        ddl_err::StdIOError(e) => eprintln!("Std IO error: {}", e),
        _ => eprintln!("Unknown error!"),
    }
    exit(1);
}
