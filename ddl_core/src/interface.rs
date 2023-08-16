use std::path::PathBuf;

use crate::{
    entry::{EntrySet, EntryUnit},
    errors::{DDLError, EntryKeyNotFound},
    fs,
    item::{ItemImportance, ItemList, ItemUnit},
};

fn get_all_entry(path: PathBuf) -> Result<Vec<EntryUnit>, DDLError> {
    let entry_set = EntrySet::read_entry(path)?;
    Ok(entry_set.entries)
}

pub fn default_dir() -> Result<PathBuf, DDLError> {
    fs::ensure_data_dir()
}

pub fn get_entry_list(path: PathBuf) -> Result<Vec<(usize, String)>, DDLError> {
    let entries = get_all_entry(path)?;
    let mut res = Vec::new();
    for entry in entries {
        res.push((entry.id, entry.key));
    }
    Ok(res)
}

fn get_item_list_by_entry(path: PathBuf, entry: &EntryUnit) -> Result<Vec<ItemUnit>, DDLError> {
    let item_list = ItemList::read_list(path, entry)?;
    Ok(item_list.items)
}

pub struct ItemUnitDisp {
    pub importance: String,
    pub ddl: String,
    pub rest: String,
    pub percent: f64,
    pub label: String,
    pub over: bool,
    pub key: String,
}

impl ItemUnitDisp {
    pub fn new(unit: ItemUnit, key: String) -> Self {
        let importace = match unit.importance {
            ItemImportance::Least => "No Importance",
            ItemImportance::Less => "Tiny Importance",
            ItemImportance::Normal => "Normal Importance",
            ItemImportance::More => "Big Importance",
            ItemImportance::Most => "Huge Importance",
        };
        let importance = importace.to_string();

        let ddl = format!(
            "{} {:02}:{:02}",
            unit.time.ddl.date(),
            unit.time.ddl.hour(),
            unit.time.ddl.minute()
        );

        let rest = unit.time.delta();
        let rest = (rest.whole_days(), rest.whole_hours(), rest.whole_minutes());
        let rest = format!(
            "{} day{} {} hour{} {} minute{}",
            rest.0,
            if rest.0 > 1 { "s" } else { "" },
            rest.1 % 24,
            if rest.1 % 24 > 1 { "s" } else { "" },
            rest.2 % 60,
            if rest.2 % 60 % 24 > 1 { "s" } else { "" },
        );

        let percent = unit.time.percent();
        let label = unit.label;
        let over = unit.over;

        ItemUnitDisp {
            importance,
            ddl,
            rest,
            percent,
            label,
            over,
            key,
        }
    }
}

pub fn get_item_list_by_key(
    path: PathBuf,
    entry_key: &String,
) -> Result<Vec<ItemUnitDisp>, DDLError> {
    let entry = EntrySet::read_entry(path.clone())?;
    let entry = entry
        .select_from_key(entry_key)
        .ok_or(EntryKeyNotFound {})?;
    let mut item_list = get_item_list_by_entry(path, entry)?;
    item_list.sort();
    let mut res = Vec::new();
    for item in item_list {
        res.push(ItemUnitDisp::new(item, entry_key.clone()));
    }
    Ok(res)
}

pub fn get_item_whole(path: PathBuf) -> Result<Vec<(String, Vec<ItemUnitDisp>)>, DDLError> {
    let entry = EntrySet::read_entry(path.clone())?;
    let mut res_vec = Vec::new();
    for entry in entry.entries {
        let mut item_list = get_item_list_by_entry(path.clone(), &entry)?;
        item_list.sort();
        let mut res = Vec::new();
        for item in item_list {
            res.push(ItemUnitDisp::new(item, entry.key.clone()));
        }
        res_vec.push((entry.key.clone(), res));
    }
    Ok(res_vec)
}
