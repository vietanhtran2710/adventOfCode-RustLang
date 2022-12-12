use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut start = -1; let mut count = 0;
    let lines = contents.split('\n');
    for line in lines {
        let value = line.parse::<i32>().unwrap();
        if start != -1 {
            if value > start {
                count = count + 1;
            }
        }
        start = value;
    }
    println!("{}", count);
}