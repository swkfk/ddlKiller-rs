use ddl_core::{entry::EntrySet, fs};

#[test]
fn test_data_dir() {
    fs::ensure_data_dir().unwrap();
}

#[test]
fn test_empty_entry() {
    let s = EntrySet::serialize(&EntrySet::new()).unwrap();
    assert_eq!(s, "entries = []\n");
    EntrySet::deserialize(s).unwrap();
}

#[test]
fn test_read_entryset() {
    let path = fs::ensure_data_dir().unwrap();
    EntrySet::read_entry(path).unwrap();
}

#[test]
fn test_write_to_file() {
    let dir = std::env::temp_dir().join("ddl-killer-test-write-to-file");
    if !dir.exists() {
        std::fs::create_dir(dir.clone()).unwrap();
    }
    let mut set = EntrySet {
        entries: Vec::new(),
    };
    set.append("key1".into(), "the first key".into());
    set.append("key2".into(), "the second key".into());
    set.write_entry(dir.clone()).unwrap();

    let read = EntrySet::read_entry(dir.clone()).unwrap();
    assert_eq!(read.select_from_id(1).unwrap().key, "key1");
    assert_eq!(read.select_from_key(&"key2".into()).unwrap().key, "key2");
    assert_eq!(read.entries[0].key, "key1");
    assert_eq!(read.entries[1].key, "key2");
    assert!(read.select_from_id(0).is_none());
    assert!(read.select_from_id(3).is_none());
    assert!(read.select_from_key(&"key".into()).is_none());

    set.select_from_id_mut(1).unwrap().disable();
    set.select_from_key_mut(&"key2".into()).unwrap().key = "KEY2".into();
    set.append("key3".into(), "the third key".into());
    set.write_entry(dir.clone()).unwrap();

    let mut read = EntrySet::read_entry(dir.clone()).unwrap();
    assert_eq!(read.select_from_id(2).unwrap().key, "KEY2");
    assert!(read.select_from_key(&"key1".into()).is_none());
    assert!(read.select_from_id_mut(1).is_none());
    assert_eq!(read.select_from_id_mut(3).unwrap().key, "key3");
}
