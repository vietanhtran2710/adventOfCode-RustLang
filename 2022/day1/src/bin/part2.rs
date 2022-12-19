use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');
    let mut calories: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for line in lines {
        if line.len() != 0 {
            current_calories += line.parse::<i32>().unwrap();
        }
        else {
            calories.push(current_calories);
            current_calories = 0;
        }
    }
    calories.push(current_calories);
    calories.sort_by(|a, b| b.cmp(a));
    println!("{}", calories[0] + calories[1] + calories[2]);
}
