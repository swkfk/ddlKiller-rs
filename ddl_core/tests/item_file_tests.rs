use ddl_core::{
    entry::EntryUnit,
    item::{self, ItemList},
};
use time::{Duration, OffsetDateTime};

#[test]
fn test_ser_de_itemunit() {
    let unit1 = item::ItemUnit::new(
        item::ItemImportance::Normal,
        "label".into(),
        OffsetDateTime::now_utc() + Duration::HOUR,
    );
    let unit2 = item::ItemUnit::new(
        item::ItemImportance::More,
        "label".into(),
        OffsetDateTime::now_utc() + Duration::DAY,
    );
    let ser = toml::to_string(&unit1).unwrap();
    let de: item::ItemUnit = toml::from_str(&ser).unwrap();
    assert_eq!(de, unit1);

    let mut set = item::ItemList::new();
    set.items.push(unit1);
    set.items.push(unit2);
    let ser = toml::to_string(&set).unwrap();
    let de: item::ItemList = toml::from_str(&ser).unwrap();
    assert_eq!(set.items[1], de.items[1]);
}

#[test]
fn test_write_to_file() {
    let dir = std::env::temp_dir().join("ddl-killer-item-test-write-to-file");
    if !dir.exists() {
        std::fs::create_dir(dir.clone()).unwrap();
    }
    let entry = EntryUnit {
        id: 1,
        desc: "".into(),
        key: "key1".into(),
        enabled: true,
    };
    let mut list = ItemList::new();
    list.append(
        item::ItemImportance::Normal,
        "label1".into(),
        time::macros::datetime!(2023 - 09 - 01 0:00 +8),
    );
    list.append(
        item::ItemImportance::Normal,
        "label2".into(),
        time::macros::datetime!(2022 - 10 - 01 0:00 +8),
    );

    list.write_list(dir.clone(), &entry).unwrap();

    let read = ItemList::read_list(dir.clone(), &entry).unwrap();
    assert_eq!(2, read.items.len());
    assert!(read.items[1].time.timeout());
    assert_eq!(read.items[1].time.percent(), 100.0);

    list.select_from_id_mut(2).unwrap().set_over();
    assert_eq!(list.select_from_id(1).unwrap().label, "label1");

    list.write_list(dir.clone(), &entry).unwrap();
    let read = ItemList::read_list(dir.clone(), &entry).unwrap();
    assert!(read.select_from_id(3).is_none());
    assert!(read.select_from_id(2).unwrap().over);
}
