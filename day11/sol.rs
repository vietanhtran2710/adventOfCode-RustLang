use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let lines = data.split("\n");
    for line in lines {
        let row: Vec<u32> = line.chars().map(|s| s.to_digit(10).unwrap()).collect();
        matrix.push(row);
    }
    let mut count: i32 = 0;
    let dir_x: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_y: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];
    for _step in 0..100 {
        let mut flashed = vec![[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                let mut queue: Vec<Vec<usize>> = vec![vec![x, y]];
                while queue.len() != 0 {
                    let (i, j) = (queue[0][0], queue[0][1]); queue.remove(0);
                    if !flashed[i][j] {
                        matrix[i][j] += 1;
                        if matrix[i][j] > 9 {
                            count += 1; matrix[i][j] = 0; flashed[i][j] = true;
                            for ind in 0..8 {
                                if (i as i32 + dir_x[ind as usize] >= 0) && (j as i32 + dir_y[ind as usize] >= 0) {
                                    let ind_x: usize = (i as i32 + dir_x[ind as usize]) as usize;
                                    let ind_y: usize = (j as i32 + dir_y[ind as usize]) as usize;
                                    if (ind_x < 10) && (ind_y < 10) {
                                        queue.push(vec![ind_x, ind_y]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let data = input("input.txt"); 
    println!("{}", process(data));
}
