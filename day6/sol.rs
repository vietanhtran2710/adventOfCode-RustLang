use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let mut vector: Vec<i32> = data.split(",").map(|s| s.parse().expect("error")).collect();
    for _i in 1..=80 {
        let length = vector.len();
        for j in 0..length {
            if vector[j] == 0 {
                vector[j] = 6;
                vector.push(8);
            }
            else {
                vector[j] -= 1;
            }
        }
    }
    vector.len() as i32
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
