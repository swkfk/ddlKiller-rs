use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::{Duration, OffsetDateTime};

use crate::errors::DDLError;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ItemImportance {
    Least,
    Less,
    Normal,
    More,
    Most,
}

impl ItemImportance {
    pub fn new() -> ItemImportance {
        ItemImportance::Normal
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq)]
pub struct ItemTime {
    pub setup: OffsetDateTime,
    pub ddl: OffsetDateTime,
}

impl ItemTime {
    pub fn new(ddl: OffsetDateTime) -> ItemTime {
        ItemTime {
            setup: OffsetDateTime::now_utc(),
            ddl,
        }
    }

    pub fn timeout(&self) -> bool {
        self.ddl <= OffsetDateTime::now_utc()
    }

    pub fn delta(&self) -> Duration {
        let now = OffsetDateTime::now_utc();
        if self.ddl <= now {
            Duration::ZERO
        } else {
            self.ddl - now
        }
    }

    pub fn percent(&self) -> f64 {
        let rest = self.delta();
        let total = self.ddl - self.setup;
        if rest.is_zero() {
            100.0
        } else {
            (total - rest) / total * 100.0
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct ItemUnit {
    pub importance: ItemImportance,
    pub time: ItemTime,
    pub label: String,
    pub over: bool,
}

impl ItemUnit {
    pub fn new(importance: ItemImportance, label: String, ddl: OffsetDateTime) -> ItemUnit {
        ItemUnit {
            importance,
            time: ItemTime::new(ddl),
            label,
            over: false,
        }
    }

    pub fn set_over(&mut self) {
        self.over = true;
    }
}

impl Ord for ItemUnit {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.over && !other.over {
            return Ordering::Greater;
        } else if !self.over && other.over {
            return Ordering::Less;
        }
        if self.time.ddl != other.time.ddl {
            return self.time.ddl.cmp(&other.time.ddl);
        }
        return other.importance.cmp(&self.importance);
    }
}

impl PartialOrd for ItemUnit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ItemUnit {
    fn eq(&self, other: &Self) -> bool {
        self.importance == other.importance
            && self.time.ddl == other.time.ddl
            && self.over == other.over
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ItemList {
    pub items: Vec<ItemUnit>,
}

impl ItemList {
    pub fn new() -> ItemList {
        ItemList { items: Vec::new() }
    }

    pub fn push_unit(&mut self, new: ItemUnit) -> () {
        self.items.push(new);
    }

    pub fn append(&mut self, importance: ItemImportance, label: String, ddl: OffsetDateTime) -> () {
        self.push_unit(ItemUnit::new(importance, label, ddl));
    }

    pub fn select_from_id(&self, id: usize) -> Option<&ItemUnit> {
        if id > 0 && id <= self.items.len() {
            Some(&self.items[id - 1])
        } else {
            None
        }
    }

    pub fn select_from_id_mut(&mut self, id: usize) -> Option<&mut ItemUnit> {
        if id > 0 && id <= self.items.len() {
            Some(&mut self.items[id - 1])
        } else {
            None
        }
    }

    pub fn serialize(&self) -> Result<String, DDLError> {
        Ok(toml::to_string(self)?)
    }

    pub fn deserialize(s: String) -> Result<ItemList, DDLError> {
        Ok(toml::from_str(&s)?)
    }
}
