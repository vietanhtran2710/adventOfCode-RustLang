use std::fs;

fn input(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Can't read file");
    return data;
}

fn process(data: String) -> i32 {
    let (mut depth, mut horiz, mut aim) = (0, 0, 0);
    let lines = data.split('\n');
    for line in lines {
        let vec = line.split(" ").collect::<Vec<&str>>();
        let value = vec[1].parse::<i32>().unwrap();
        if vec[0] == "forward" {
            horiz += value;
            depth += aim * value;
        }
        else if vec[0] == "down" {
            aim += value;
        }
        else {
            aim -= value;
        }
    }
    return horiz * depth;
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}