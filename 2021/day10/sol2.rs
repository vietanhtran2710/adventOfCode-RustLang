use std::fs;
use std::collections::HashMap;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i64 {
    let lines = data.split("\n");
    let closing = HashMap::from([
        (')', '('), (']', '['), ('}', '{'), ('>', '<'),
    ]);
    let point_map = HashMap::from([
        ('(', 1), ('[', 2), ('{', 3), ('<', 4),
    ]);
    let mut points: Vec<i64> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut valid = true;
        for ch in line.chars() {
            if "({[<".contains(ch) {
                stack.push(ch);
            }
            else {
                if stack.pop().unwrap() != *closing.get(&ch).unwrap() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            let mut point = 0;
            while stack.len() != 0 {
                point = point * 5 + point_map.get(&stack.pop().unwrap()).unwrap();
            }
            points.push(point);
        }
    }
    points.sort();
    points[points.len() / 2]
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
