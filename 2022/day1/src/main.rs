use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in lines {
        if line.len() != 0 {
            current_calories += line.parse::<i32>().unwrap();
        }
        else {
            max_calories = max_calories.max(current_calories);
            current_calories = 0;
        }
    }
    max_calories = max_calories.max(current_calories);
    println!("{}", max_calories);
}
