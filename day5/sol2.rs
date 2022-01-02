use std::fs;
use std::cmp;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let mut diagram: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    let lines = data.split("\n");
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        let point1: Vec<i32> = points[0].split(",").map(|s| s.parse().expect("error")).collect();
        let point2: Vec<i32> = points[1].split(",").map(|s| s.parse().expect("error")).collect();
        if point1[0] == point2[0] {
                for i in cmp::min(point1[1], point2[1])..=cmp::max(point1[1], point2[1]) {
                    diagram[point1[0] as usize][i as usize] += 1;
                }
        }
        else if point1[1] == point2[1] {
            for i in cmp::min(point1[0], point2[0])..=cmp::max(point1[0], point2[0]) {
                diagram[i as usize][point1[1] as usize] += 1;
            }
        }
        else if (point1[0] - point2[0]).abs() == (point1[1] - point2[1]).abs() {
            let (mut dir0, mut dir1) = (1, 1);
            if point1[0] > point2[0] {
                dir0 = -1;
            }
            if point1[1] > point2[1] {
                dir1 = -1;
            }
            for i in 0..=(point1[0] - point2[0]).abs() {
                diagram[(point1[0] + i * dir0) as usize][(point1[1] + i * dir1) as usize] += 1;
            }
        }
    }
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if diagram[i][j] > 1 {
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
