use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let vector: Vec<i32> = data.split(",").map(|s| s.parse().expect("error")).collect();
    let mut min = 1000 * 2000;
    for i in 0..=1981 {
        let mut fuel = 0;
        for item in &vector {
            fuel += (item - i).abs();
        }
        if fuel < min {
            min = fuel;
        }
    }
    min
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
