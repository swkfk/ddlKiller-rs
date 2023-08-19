use ddl_core::errors::ArgsParseError as ParserErrorStruct;
use ddl_core::errors::DDLError as ddl_err;

pub enum EntrySelect {
    All,
    ByKey(String),
    // ById(usize),
}

pub enum NewType {
    Entry,
    Item,
}

pub enum OverType {
    Entry(String),
    Item(String, usize),
}

pub enum ArgOps {
    Show(EntrySelect),
    List(EntrySelect),
    New(NewType),
    Over(OverType),
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
        } else if cmd[0] == "list" {
            match cmd.get(1) {
                None => Ok(ArgOps::List(EntrySelect::All)),
                Some(k) => Ok(ArgOps::List(EntrySelect::ByKey(k.to_string()))),
            }
        } else if cmd[0] == "new" {
            match cmd.get(1) {
                Some(&"entry") => Ok(ArgOps::New(NewType::Entry)),
                Some(&"ddl") => Ok(ArgOps::New(NewType::Item)),
                Some(_) => Err(ddl_err::ArgsParseError(ParserErrorStruct {
                    position: cmd[1].to_string(),
                    cause: "Unknown field",
                })),
                None => Err(ddl_err::ArgsParseError(ParserErrorStruct {
                    position: cmd[0].to_string(),
                    cause: "Expected field",
                })),
            }
        } else if cmd[0] == "over" {
            match (cmd.get(1), cmd.get(2)) {
                (Some(k), Some(v)) => {
                    if let Ok(i) = v.parse() {
                        Ok(ArgOps::Over(OverType::Item(k.to_string(), i)))
                    } else {
                        Err(ddl_err::ArgsParseError(ParserErrorStruct {
                            position: cmd[2].to_string(),
                            cause: "Invalid id number",
                        }))
                    }
                }
                (Some(k), None) => Ok(ArgOps::Over(OverType::Entry(k.to_string()))),
                (None, _) => Err(ddl_err::ArgsParseError(ParserErrorStruct {
                    position: "<Eof>".to_string(),
                    cause: "Expected entry kry or with ddl id",
                })),
            }
        } else {
            Err(ddl_err::ArgsParseError(ParserErrorStruct {
                position: cmd[0].to_string(),
                cause: "Unknown command",
            }))
        }
    }
}
