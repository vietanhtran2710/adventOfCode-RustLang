use std::fs;
use std::collections::HashMap;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u128 {
    let lines = data.split("\n");
    let mut map: HashMap<String, String> = HashMap::new();
    let mut template: &str = "";
    for (index, line) in lines.enumerate() {
        if index == 0 {
            template = line;
        }
        else if index >= 2 {
            let items: Vec<&str> = line.split(" -> ").collect();
            map.insert(items[0].to_owned(), items[1].to_owned());
        }
    }
    let mut count: HashMap<String, u128> = HashMap::new();
    for i in 1..template.len() {
        let key = &template[i - 1..i + 1];
        if count.contains_key(&key.to_owned()) {
            *count.get_mut(&key.to_owned()).unwrap() += 1;
        }
        else {
            count.insert(key.to_owned(), 1);
        }
    }
    for key in map.keys() {
        if !count.contains_key(key) {
            count.insert(key.to_string(), 0);
        }
    }
    for _i in 0..40 {
        let mut new_count: HashMap<String, u128> = HashMap::new();
        for key in map.keys() {
            let old_value = count.get(key).unwrap(); let new_char = map.get(key).unwrap();
            let mut new_key1: String = key.chars().nth(0).unwrap().to_string();
            new_key1 = new_key1 + new_char;
            let new_key2: String = new_char.to_owned() + &key.chars().nth(1).unwrap().to_string();
            if new_count.contains_key(&new_key1) {
                *new_count.get_mut(&new_key1).unwrap() += old_value;
            }
            else {
                new_count.insert(new_key1, *old_value);
            }
            if new_count.contains_key(&new_key2) {
                *new_count.get_mut(&new_key2).unwrap() += old_value;
            }
            else {
                new_count.insert(new_key2, *old_value);
            }
        }
        for key in map.keys() {
            if !new_count.contains_key(key) {
                new_count.insert(key.to_string(), 0);
            }
        }
        count = new_count.clone();
    }
    let mut count_char: HashMap<char, u128> = HashMap::new();
    for key in count.keys() {
        let mut ch = key.chars().nth(0).unwrap();
        if !count_char.contains_key(&ch) {
            count_char.insert(ch, *count.get(key).unwrap());
        }
        else {
            *count_char.get_mut(&ch).unwrap() += count.get(key).unwrap();
        }
        ch = key.chars().nth(1).unwrap();
        if !count_char.contains_key(&ch) {
            count_char.insert(ch, *count.get(key).unwrap());
        }
        else {
            *count_char.get_mut(&ch).unwrap() += count.get(key).unwrap();
        }
    }
    let clone_map:  HashMap<char, u128> = count_char.clone();
    for key in clone_map.keys() {
        *count_char.get_mut(key).unwrap() = count_char.get(key).unwrap() / 2;
    }
    *count_char.get_mut(&template.chars().nth(0).unwrap()).unwrap() += 1;
    *count_char.get_mut(&template.chars().nth(template.len() - 1).unwrap()).unwrap() += 1;
    *count_char.values().max().unwrap() - *count_char.values().min().unwrap()
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
