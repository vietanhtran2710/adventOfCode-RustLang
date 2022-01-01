use std::fs;

fn input(path: &str) -> String {
    let data = fs::read_to_string(path).expect("Can't read file");
    return data;
}

fn process(data: String) -> i32 {
    let (mut depth, mut horiz) = (0, 0);
    let lines = data.split('\n');
    for line in lines {
        let vec = line.split(" ").collect::<Vec<&str>>();
        if vec[0] == "forward" {
            horiz += vec[1].parse::<i32>().unwrap();
        }
        else if vec[0] == "down" {
            depth += vec[1].parse::<i32>().unwrap();
        }
        else {
            depth -= vec[1].parse::<i32>().unwrap();
        }
    }
    return horiz * depth;
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}