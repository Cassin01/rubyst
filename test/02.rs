use std::collections::BTreeMap;

let mut map = BTreeMap::new();

map.insert("lisp", 1958);
map.insert("c", 1972);
map.insert("rust", 2006);

assert_eq!(map["lisp"], 1958);