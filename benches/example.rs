use cargo_balls::balls_main;
use std::collections::{BTreeMap, HashMap};
use std::hint::black_box;

fn hash_map() {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert(i, i);
    }
    black_box(map);
}

fn btree_map() {
    let mut map = BTreeMap::new();
    for i in 0..1000 {
        map.insert(i, i);
    }
    black_box(map);
}

balls_main!(title: "HashMap vs. BTreeMap sequential inserts"; hash_map, btree_map);
