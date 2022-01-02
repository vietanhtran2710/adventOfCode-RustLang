use std::fs;
use std::collections::HashMap;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect()
}

fn decode(input: &str, output: &str) -> i32 {
    let signals: Vec<&str> = input.split(" ").collect();
    let digits: Vec<&str> = output.split(" ").collect();
    let mut decode_map = HashMap::new();
    let mut one_chars: &str = "";
    let mut five_segments: Vec<&str> = vec![""; 0];
    let mut six_segments: Vec<&str> = vec![""; 0];
    let full_segments = "abcdefg"; let mut missing_segment = 'a';
    let (mut two, mut three, mut five) = ("", "", "");
    for item in signals {
        if item.len() == 2 {
            decode_map.insert(sort(item), 1);
            one_chars = item;
        }
        else if item.len() == 4 {
            decode_map.insert(sort(item), 4);
        }
        else if item.len() == 3 {
            decode_map.insert(sort(item), 7);
        }
        else if item.len() == 7 {
            decode_map.insert(sort(item), 8);
        }
        else if item.len() == 6 {
            six_segments.push(item);
        }
        else {
            five_segments.push(item);
        }
    }
    for (index, item) in six_segments.iter().enumerate() {
        if !((item.contains(one_chars.chars().nth(0).unwrap())) && (item.contains(one_chars.chars().nth(1).unwrap()))) {
            decode_map.insert(sort(item), 6);
            for ch in full_segments.chars() {
                if !item.contains(ch) {
                    missing_segment = ch; break;
                }
            }
            six_segments.remove(index);
            break;
        }
    }
    for (index, item) in five_segments.iter().enumerate() {
        if (item.contains(one_chars.chars().nth(0).unwrap())) && (item.contains(one_chars.chars().nth(1).unwrap())) {
            decode_map.insert(sort(item), 3);
            three = item;
            five_segments.remove(index);
            break;
        }
    }
    for (index, item) in five_segments.iter().enumerate() {
        if item.contains(missing_segment) {
            decode_map.insert(sort(item), 2);
            two = item;
            five_segments.remove(index);
            break;
        }
    }
    five = five_segments[0];
    decode_map.insert(sort(five_segments[0]), 5);
    for ch in two.chars() {
        if (!(three.contains(ch))) && !(five.contains(ch)) {
            missing_segment = ch; break;
        }
    }
    for (index, item) in six_segments.iter().enumerate() {
        if !item.contains(missing_segment) {
            decode_map.insert(sort(item), 9);
            six_segments.remove(index);
            break;
        }
    }
    decode_map.insert(sort(six_segments[0]), 0);
    let mut result = 0;
    for item in digits {
        result = result * 10 + decode_map.get(&sort(item)).unwrap();
    }
    result
}

fn process(data: String) -> i32 {
    let mut result = 0;
    let lines = data.split("\n");
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let value = decode(parts[0], parts[1]);
        result += value;
    }
    result
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
