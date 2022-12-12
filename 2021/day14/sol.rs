use std::fs;
use std::collections::HashMap;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let lines = data.split("\n");
    let mut map: HashMap<String, &str> = HashMap::new();
    let mut template: Vec<&str> = vec![""; 1];
    for (index, line) in lines.enumerate() {
        if index == 0 {
            template = line.split("").collect();
        }
        else if index >= 2 {
            let items: Vec<&str> = line.split(" -> ").collect();
            map.insert(items[0].to_owned(), items[1]);
        }
    }
    let mut index = 0;
    while index < template.len() {
        while (index < template.len()) && (template[index] == "") {
            template.remove(index);
        }
        index += 1;
    }
    for _i in 0..4 {
        let mut new_str: Vec<&str> = vec![template[0]];
        for j in 1..template.len() {
            let mut key: String = template[j - 1].to_owned();
            key.push_str(template[j]);
            if map.contains_key(&key) {
                let mut add: Vec<&str> = map.get(&key).unwrap().split("").collect();
                let mut ind = 0;
                while ind < add.len() {
                    while (ind < add.len()) && (add[ind] == "") {
                        add.remove(ind);
                    }
                    ind += 1;
                }
                new_str.append(&mut add);
            }
            new_str.push(template[j]);
        }
        template = new_str.clone();
    }
    let mut count: HashMap<&str, i32> = HashMap::new();
    for item in template {
        if count.contains_key(item) {
            *count.get_mut(item).unwrap() += 1;
        }
        else {
            count.insert(item, 1);
        }
    }
    count.values().max().unwrap() - count.values().min().unwrap()
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
