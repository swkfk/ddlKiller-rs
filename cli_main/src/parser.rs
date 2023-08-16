use ddl_core::errors::ArgsParseError as ParserErrorStruct;
use ddl_core::errors::DDLError as ddl_err;

pub enum EntrySelect {
    All,
    ByKey(String),
    // ById(usize),
}

pub enum ArgOps {
    Show(EntrySelect),
}

impl ArgOps {
    pub fn parse(args: Vec<String>) -> Result<ArgOps, ddl_err> {
        if args.len() < 2 {
            return Err(ddl_err::ArgsParseError(ParserErrorStruct {
                position: "<Eof>".to_string(),
                cause: "Expected detailed command",
            }));
        }
        let cmd = &args[1];
        let cmd: Vec<&str> = cmd.split("/").collect();
        if cmd[0] == "show" {
            match cmd.get(1) {
                None => Ok(ArgOps::Show(EntrySelect::All)),
                Some(k) => Ok(ArgOps::Show(EntrySelect::ByKey(k.to_string()))),
            }
        } else {
            Err(ddl_err::ArgsParseError(ParserErrorStruct {
                position: cmd[0].to_string(),
                cause: "Unknown command",
            }))
        }
    }
}
