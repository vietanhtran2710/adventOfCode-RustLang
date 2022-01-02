use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let lines = data.split("\n");
    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        for word in parts[1].split(" ") {
            if (word.len() == 2) || (word.len() == 3) || (word.len() == 4) || (word.len() == 7) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
