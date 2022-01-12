use std::fs;
use std::collections::HashSet;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let lines = data.split("\n");
    let mut risk: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row: Vec<u32> = line.chars().map(|s| s.to_digit(10).unwrap()).collect();
        for i in 1..=4 {
            let len = row.len();
            for j in len - 100..len {
                let mut new_value = row[j] + 1;
                if new_value > 9 {
                    new_value = 1;
                }
                row.push(new_value);
            }
        }
        risk.push(row);
    }
    println!("Done expanding column");
    for i in 1..=4 {
        let len = risk.len();
        for j in len - 100..len {
            let mut new_row: Vec<u32> = Vec::new();
            for item in &risk[j] {
                let mut new_value = item + 1;
                if new_value > 9 {
                    new_value = 1;
                }
                new_row.push(new_value);
            }
            risk.push(new_row);
        }
    }
    println!("Done expanding, {} {}", risk.len(), risk[0].len());

    fn adjacent(point1: usize, point2: usize) -> bool {
        let (row1, col1) = (point1 / 500, point1 % 500);
        let (row2, col2) = (point2 / 500, point2 % 500);
        if row1 == row2 {
            if (col1 as i32 - col2 as i32).abs() == 1 {
                return true;
            }
        }
        else if col2 == col1 {
            if (row1 as i32 - row2 as i32).abs() == 1 {
                return true;
            }
        }
        return false;
    }

    let mut distance = [250000 * 9; 250000]; distance[0] = 0;
    let mut checked: HashSet<usize> = HashSet::new();
    let mut unchecked: HashSet<usize> = HashSet::new();
    let mut reduced: HashSet<usize> = HashSet::new();
    reduced.insert(0);
    for i in 0..250000 {
        unchecked.insert(i);
    }
    println!("Done Initialized");
    while !checked.contains(&249999) {
        let (mut index, mut min) = (0, 250000 * 9);
        for i in &unchecked {
            if distance[*i] < min {
                index = *i; min = distance[*i];
            }
        }
        checked.insert(index);
        println!("{}", checked.len());
        unchecked.remove(&index);
        for i in &unchecked {
            if adjacent(index, *i) {
                let value = risk[i / 500][i % 500];
                if distance[index] + value < distance[*i] {
                    distance[*i] = distance[index] + value;
                    if distance[*i] != 250000 * 9 {
                        reduced.insert(*i);
                    }
                }
            }
        }
    }
    distance[249999]
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
