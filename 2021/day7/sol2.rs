use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i128 {
    let vector: Vec<i32> = data.split(",").map(|s| s.parse().expect("error")).collect();
    let mut min: i128 = 3000000 * 1000;
    for i in 0..=1981 {
        let mut fuel: i128 = 0;
        for item in &vector {
            let distance: i128 = (item - i).abs().into();
            fuel += ((distance + 1) * distance) / 2;
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
