use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u32 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let lines = data.split("\n");
    for line in lines {
        let row: Vec<u32> = line.chars().map(|s| s.to_digit(10).unwrap()).collect();
        matrix.push(row);
    }
    let dir_x: Vec<i32> = vec![-1, 0, 1, 0];
    let dir_y: Vec<i32> = vec![0, 1, 0, -1];
    let mut result = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let mut low_point = true;
            for ind in 0..4 {
                if (i as i32 + dir_x[ind as usize] >= 0) && (j as i32 + dir_y[ind as usize] >= 0) {
                    let ind_x: usize = (i as i32 + dir_x[ind as usize]) as usize;
                    let ind_y: usize = (j as i32 + dir_y[ind as usize]) as usize;
                    if (ind_x < matrix.len()) && (ind_y < row.len()) {
                        if value >= &matrix[ind_x][ind_y] {
                            low_point = false; break;
                        }
                    }
                }
            }
            if low_point {
                result += value + 1;
            }
        }
    }
    result
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
