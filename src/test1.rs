use std::collections::HashMap;

fn main() {
    test_hash_map();
}

fn test_vec() {
    let list = vec![1, 2, 3, 4, 5];
    for i in list.iter() {
        println!("{}", i);
    }
}

fn test_hash_map() {
    let mut hash_map = HashMap::new();
    hash_map.insert("Monday", 1);
    hash_map.insert("Tuesday", 2);
    hash_map.insert("Wednesday", 3);

    println!("{:?}", hash_map.get("Tuesday").unwrap())
}
