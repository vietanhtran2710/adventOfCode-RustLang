use std::fs;
use std::collections::HashSet;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let lines = data.split("\n");
    let mut risk: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let row: Vec<u32> = line.chars().map(|s| s.to_digit(10).unwrap()).collect();
        risk.push(row);
    }

    fn adjacent(point1: usize, point2: usize) -> bool {
        let (row1, col1) = (point1 / 100, point1 % 100);
        let (row2, col2) = (point2 / 100, point2 % 100);
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

    let mut distance = [10000; 10000]; distance[0] = 0;
    let mut checked: HashSet<usize> = HashSet::new();
    let mut unchecked: HashSet<usize> = HashSet::new();
    for i in 0..10000 {
        unchecked.insert(i);
    }
    while !checked.contains(&9999) {
        let (mut index, mut min) = (0, 10000);
        for i in &unchecked {
            if distance[*i] < min {
                index = *i; min = distance[*i];
            }
        }
        checked.insert(index);
        unchecked.remove(&index);
        for i in &unchecked {
            if adjacent(index, *i) {
                let value = risk[i / 100][i % 100];
                if distance[index] + value < distance[*i] {
                    distance[*i] = distance[index] + value;
                }
            }
        }
    }
    distance[9999]
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
