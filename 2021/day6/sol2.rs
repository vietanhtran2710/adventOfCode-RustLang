use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i128 {
    let vector: Vec<i32> = data.split(",").map(|s| s.parse().expect("error")).collect();
    let mut count: Vec<i128> = [0; 9].to_vec();
    for item in vector {
        count[item  as usize] += 1;
    }
    for _i in 1..=256 {
        let mut new_count: Vec<i128> = [0; 9].to_vec();
        for j in 1..=8 {
            new_count[6] += count[0];
            new_count[(j - 1) as usize] = count[j as usize];
            new_count[8] = count[0];
        }
        count = new_count.clone();
    }
    let mut result: i128 = 0;
    for i in 0..9 {
        result += count[i as usize];
    }
    result
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
