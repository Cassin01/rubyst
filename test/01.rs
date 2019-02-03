use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut map = HashMap::new();


    map.insert("Lisp", 1958);
    map.insert("c", 1972);
    map.insert("rust", 2006);

    assert_eq!(map["Lisp"], 1958);

    assert_eq!(map.get("c"), Some(&1972));

    assert!(map.contains_key("rust"), true);
    read_map(map);


    let mut bmap = BTreeMap::new();

    bmap.insert("lisp", 1958);
    bmap.insert("c", 1972);
    bmap.insert("rust", 2006);
    read_bmap(bmap);
}

fn read_bmap(map: BTreeMap<&str, i64>) {
    for (k, v) in &map {
        println!("{} was born in {}", k, v);
    }
}

fn read_map(map: HashMap<&str, i64>) {
    for (k, v) in &map {
        println!("{} was born in {}", k, v);
    }
}