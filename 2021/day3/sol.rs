use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let mut vec = vec![0; 12];
    let lines = data.split('\n');
    for line in lines {
        for (index, ch) in line.chars().enumerate() {
            if ch == '1' {
                vec[index] += 1;
            }
            else {
                vec[index] -= 1;
            }
        }   
    }
    let (mut gamma, mut epsilon) = (0, 0);
    const BASE: i32 = 2;
    for (index, value) in vec.iter().enumerate() {
        let power: u32 = 12 - (index as u32) - 1; 
        if value > &0 {
            gamma += BASE.pow(power);
        }
        else {
            epsilon += BASE.pow(power);
        }
    }
    gamma * epsilon
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}