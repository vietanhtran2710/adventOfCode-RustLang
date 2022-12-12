use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let lines = data.split("\n");
    let mut paper: Vec<Vec<&str>> = vec![vec!["."; 1400]; 1400];
    let mut part1 = true;
    let (mut height, mut width) = (1400, 1400);
    for line in lines {
        if line == "" {
            part1 = false; continue;
        }
        else {
            if part1 {
                let points: Vec<usize> = line.split(",").map(|s| s.parse().expect("error")).collect();
                paper[points[1]][points[0]] = "#";
            }
            else {
                let words: Vec<&str> = line.split(" ").collect();
                let info: Vec<&str> = words[words.len() - 1].split("=").collect();
                let value: usize = info[1].parse().expect("errors");
                if info[0] == "x" {
                    for i in 0..height {
                        for j in value + 1..width {
                            if paper[i][j] == "#" {
                                paper[i][value - (j - value)] = "#";
                            }
                        }
                    }
                    width = value;
                }
                else {
                    for i in value + 1..height {
                        for j in 0..width {
                                paper[value - (i - value)][j] = "#";
                        }
                    }
                    height = value;
                }
                break;
            }
        }
    }
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if paper[i][j] == "#" {
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
