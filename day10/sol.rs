use std::fs;
use std::collections::HashMap;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let lines = data.split("\n");
    let closing = HashMap::from([
        (')', '('), (']', '['), ('}', '{'), ('>', '<'),
    ]);
    let point = HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137),
    ]);
    let mut result = 0;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for ch in line.chars() {
            if "({[<".contains(ch) {
                stack.push(ch);
            }
            else {
                if stack.pop().unwrap() != *closing.get(&ch).unwrap() {
                    result += point.get(&ch).unwrap();
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
