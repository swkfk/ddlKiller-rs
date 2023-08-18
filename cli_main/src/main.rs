use cli_main::cmd;
use cli_main::handler as err;
use cli_main::parser;

use ddl_core::errors::DDLError as ddl_err;
use ddl_core::interface as ddl;
use ddl_core::logger as log;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    log::info(format!("Args: {:?}", args));

    let data_dir = match ddl::default_dir() {
        Err(e) => err::default_dir(e),
        Ok(v) => v,
    };
    log::info(format!("Get data path: {}", data_dir.display()));

    let arg_ops = match parser::ArgOps::parse(args) {
        Err(e) => err::args_parse(e),
        Ok(v) => v,
    };

    match arg_ops {
        parser::ArgOps::Show(area) => {
            log::info("Command: show".into());
            let cmd_res: Result<(), ddl_err> = match area {
                parser::EntrySelect::All => cmd::show::select_all(&data_dir),
                parser::EntrySelect::ByKey(key) => cmd::show::select_by_key(&data_dir, key),
            };
            match cmd_res {
                Err(e) => err::cmd_show(e),
                Ok(_) => {}
            }
        }
        parser::ArgOps::List(area) => {
            log::info("Command: list".into());
            let cmd_res: Result<(), ddl_err> = match area {
                parser::EntrySelect::All => cmd::list::select_all(&data_dir),
                parser::EntrySelect::ByKey(key) => {
                    cmd::list::select_by_key(&data_dir, key, (65535, 65535))
                }
            };
            match cmd_res {
                Err(e) => err::cmd_list(e),
                Ok(_) => {}
            }
        }
        parser::ArgOps::New(field) => {
            log::info("Command: new".into());
            let cmd_res: Result<(), ddl_err> = match field {
                parser::NewType::Entry => cmd::new::entry(&data_dir),
                parser::NewType::Item => cmd::new::item(&data_dir),
            };
            match cmd_res {
                Err(e) => err::cmd_new(e),
                Ok(_) => {}
            }
        }
    };
}
