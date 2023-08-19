#[derive(Debug)]
pub struct HomeDirNotAvailable {}

impl std::error::Error for HomeDirNotAvailable {}

impl std::fmt::Display for HomeDirNotAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Home dir is not available!")
    }
}

#[derive(Debug)]
pub struct EntryKeyNotFound {}

impl std::error::Error for EntryKeyNotFound {}

impl std::fmt::Display for EntryKeyNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No such entry key!")
    }
}

#[derive(Debug)]
pub struct TerminalError {}

impl std::error::Error for TerminalError {}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Terminal error!")
    }
}

#[derive(Debug)]
pub struct ArgsParseError {
    pub position: String,
    pub cause: &'static str,
}

impl std::error::Error for ArgsParseError {}

impl std::fmt::Display for ArgsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot parse the args! (at \"{}\" because \"{}\")",
            self.position, self.cause
        )
    }
}

#[derive(Debug)]
pub struct ItemIdNotFound {}

impl std::error::Error for ItemIdNotFound {}

impl std::fmt::Display for ItemIdNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No such ddl item!",)
    }
}

#[derive(Debug)]
pub enum DDLError {
    SerializeError(toml::ser::Error),
    DeserializeError(toml::de::Error),
    StdIOError(std::io::Error),
    HomeDirNotAvailable(HomeDirNotAvailable),
    EntryKeyNotFound(EntryKeyNotFound),
    ItemIdNotFound(ItemIdNotFound),
    ArgsParseError(ArgsParseError),
    TerminalError(TerminalError),
    DateTimeError(time::error::Parse),
}

impl std::error::Error for DDLError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            DDLError::SerializeError(ref e) => Some(e),
            DDLError::DeserializeError(ref e) => Some(e),
            DDLError::StdIOError(ref e) => Some(e),
            DDLError::HomeDirNotAvailable(ref e) => Some(e),
            DDLError::EntryKeyNotFound(ref e) => Some(e),
            DDLError::ItemIdNotFound(ref e) => Some(e),
            DDLError::ArgsParseError(ref e) => Some(e),
            DDLError::TerminalError(ref e) => Some(e),
            DDLError::DateTimeError(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for DDLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DDLError::SerializeError(ref e) => e.fmt(f),
            DDLError::DeserializeError(ref e) => e.fmt(f),
            DDLError::StdIOError(ref e) => e.fmt(f),
            DDLError::HomeDirNotAvailable(ref e) => e.fmt(f),
            DDLError::EntryKeyNotFound(ref e) => e.fmt(f),
            DDLError::ItemIdNotFound(ref e) => e.fmt(f),
            DDLError::ArgsParseError(ref e) => e.fmt(f),
            DDLError::TerminalError(ref e) => e.fmt(f),
            DDLError::DateTimeError(ref e) => e.fmt(f),
        }
    }
}

impl From<toml::ser::Error> for DDLError {
    fn from(value: toml::ser::Error) -> Self {
        DDLError::SerializeError(value)
    }
}

impl From<toml::de::Error> for DDLError {
    fn from(value: toml::de::Error) -> Self {
        DDLError::DeserializeError(value)
    }
}

impl From<std::io::Error> for DDLError {
    fn from(value: std::io::Error) -> Self {
        DDLError::StdIOError(value)
    }
}

impl From<HomeDirNotAvailable> for DDLError {
    fn from(value: HomeDirNotAvailable) -> Self {
        DDLError::HomeDirNotAvailable(value)
    }
}

impl From<EntryKeyNotFound> for DDLError {
    fn from(value: EntryKeyNotFound) -> Self {
        DDLError::EntryKeyNotFound(value)
    }
}

impl From<ArgsParseError> for DDLError {
    fn from(value: ArgsParseError) -> Self {
        DDLError::ArgsParseError(value)
    }
}

impl From<TerminalError> for DDLError {
    fn from(value: TerminalError) -> Self {
        DDLError::TerminalError(value)
    }
}

impl From<time::error::Parse> for DDLError {
    fn from(value: time::error::Parse) -> Self {
        DDLError::DateTimeError(value)
    }
}

impl From<ItemIdNotFound> for DDLError {
    fn from(value: ItemIdNotFound) -> Self {
        DDLError::ItemIdNotFound(value)
    }
}
