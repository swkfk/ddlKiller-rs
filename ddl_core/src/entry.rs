use serde::{Deserialize, Serialize};

use crate::errors::DDLError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryUnit {
    pub key: String,
    id: usize,
    pub enabled: bool,
    pub desc: String,
}

// Modifier for EntryUnit
impl EntryUnit {
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntrySet {
    pub entries: Vec<EntryUnit>,
}

// Select functions for EntrySet.entries
impl EntrySet {
    pub fn select_from_id(&self, id: usize) -> Option<&EntryUnit> {
        if id > 0 && id <= self.entries.len() && self.entries[id - 1].enabled {
            Some(&self.entries[id - 1])
        } else {
            None
        }
    }

    pub fn select_from_id_mut(&mut self, id: usize) -> Option<&mut EntryUnit> {
        if id > 0 && id <= self.entries.len() && self.entries[id - 1].enabled {
            Some(&mut self.entries[id - 1])
        } else {
            None
        }
    }

    pub fn select_from_key(&self, key: &String) -> Option<&EntryUnit> {
        for unit in &self.entries {
            if &unit.key == key && unit.enabled {
                return Some(unit);
            }
        }
        return None;
    }

    pub fn select_from_key_mut(&mut self, key: &String) -> Option<&mut EntryUnit> {
        for unit in &mut self.entries {
            if &unit.key == key && unit.enabled {
                return Some(unit);
            }
        }
        return None;
    }
}

// Build functions for EntrySet.entries
impl EntrySet {
    fn push_unit(&mut self, new: EntryUnit) -> () {
        self.entries.push(new);
    }

    pub fn append(&mut self, key: String, desc: String) -> () {
        self.push_unit(EntryUnit {
            key,
            id: self.entries.len() + 1, // start from `1`
            enabled: true,
            desc,
        })
    }

    pub fn new() -> EntrySet {
        EntrySet {
            entries: Vec::new(),
        }
    }
}

// Serialize functions for EntrySet.entries
impl EntrySet {
    pub fn serialize(&self) -> Result<String, DDLError> {
        Ok(toml::to_string(self)?)
    }

    pub fn deserialize(s: String) -> Result<EntrySet, DDLError> {
        Ok(toml::from_str(&s)?)
    }
}
