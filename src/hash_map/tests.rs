use super::*;

#[test]

fn chaining() {
    let mut entry = Entry {
        key: "Isak".to_string(),
        value: "Larsson".to_string(),
        chain: None,
    };

    let res = entry.chain(Entry {
        key: "Malte".to_string(),
        value: "Blomqvist".to_string(),
        chain: None,
    });

    println!("Malte: {:?}", res);

    let res = entry.chain(Entry {
        key: "Morris".to_string(),
        value: "Hansing".to_string(),
        chain: None,
    });

    println!("Morris: {:?}", res);

    let res = entry.chain(Entry {
        key: "Morris".to_string(),
        value: "Hansing".to_string(),
        chain: None,
    });

    println!("Morris again: {:?}", res);
}

#[test]
fn get_chained_entries() {
    let mut entry = Entry {
        key: "Isak".to_string(),
        value: "Larsson".to_string(),
        chain: None,
    };

    let res = entry.chain(Entry {
        key: "Malte".to_string(),
        value: "Blomqvist".to_string(),
        chain: None,
    });

    let res = entry.chain(Entry {
        key: "Morris".to_string(),
        value: "Hansing".to_string(),
        chain: None,
    });

    let entry = entry.get_entry_by_key("Morris");
    assert_eq!(entry.unwrap().value, "Hansing".to_string());
}

#[test]
fn get_all() {
    let mut root_entry = Entry {
        key: "Isak".to_string(),
        value: "Larsson".to_string(),
        chain: None,
    };

    let res = root_entry.chain(Entry {
        key: "Malte".to_string(),
        value: "Blomqvist".to_string(),
        chain: None,
    });

    let res = root_entry.chain(Entry {
        key: "Morris".to_string(),
        value: "Hansing".to_string(),
        chain: None,
    });

    let entries = root_entry.get_all();
    assert_eq!(entries[0].key, "Isak".to_string());
    assert_eq!(entries[1].key, "Malte".to_string());
    assert_eq!(entries[2].key, "Morris".to_string());
}

#[test]
fn remove_by_key(){
    let mut root_entry = Entry {
        key: "Isak".to_string(),
        value: "Larsson".to_string(),
        chain: None,
    };

    let res = root_entry.chain(Entry {
        key: "Malte".to_string(),
        value: "Blomqvist".to_string(),
        chain: None,
    });

    let res = root_entry.chain(Entry {
        key: "Morris".to_string(),
        value: "Hansing".to_string(),
        chain: None,
    });
    root_entry.remove_by_key("Malte");
    let entries = root_entry.get_all();
    assert_eq!(entries[0].key, "Isak".to_string());
    assert_eq!(entries[1].key, "Morris".to_string());
}

#[test]
fn insert_entry() {
    let mut map = HashMap::new();
    map.insert("Isak", "Larsson");
    map.insert("Morris", "Hansing");
    map.insert("Malte", "Blomqvist");
}

#[test] 
fn resize_map() {
    let mut map = HashMap::new();
    map.insert("Isak", "Larsson");
    map.insert("Morris", "Hansing");
    map.insert("Malte", "Blomqvist");
    map.insert("Iak", "Larsson");
    map.insert("Mrris", "Hansing");
    map.insert("Mlte", "Blomqvist");
    map.insert("Isaak", "Larsson");
    map.insert("Isaaak", "Larsson");
    assert_eq!(map.len(), 20);

}

#[test]
fn get_value_with_key() {
    let mut map = HashMap::new();
    map.insert("Isak", "Larsson");
    map.insert("Morris", "Hansing");
    map.insert("Malte", "Blomqvist");
    let value = map.get("Malte");
    assert_eq!(value.unwrap(), "Blomqvist");
}

#[test]
fn remove(){
    let mut map = HashMap::new();
    map.insert("Isak", "Larsson");
    map.insert("Morris", "Hansing");
    map.insert("Malte", "Blomqvist");
    map.remove("Malte");
    let load = map.load;
    assert_eq!(load, 2);
}