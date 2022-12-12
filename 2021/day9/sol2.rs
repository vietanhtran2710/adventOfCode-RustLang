use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> u128 {

    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let lines = data.split("\n");
    for line in lines {
        let row: Vec<u32> = line.chars().map(|s| s.to_digit(10).unwrap()).collect();
        matrix.push(row);
    }
    let mut tree: Vec<usize> = Vec::new();
    for i in 0..matrix[0].len() * matrix.len() {
        tree.push(i);
    }
    let dir_x: Vec<usize> = vec![0, 1];
    let dir_y: Vec<usize> = vec![1, 0];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != 9 {
                for ind in 0..2 {
                    let ind_x: usize = i + dir_x[ind as usize];
                    let ind_y: usize = j + dir_y[ind as usize];
                    if (ind_x < matrix.len()) && (ind_y < matrix[0].len()) {
                        if matrix[ind_x][ind_y] != 9 {
                            if tree[ind_x * matrix[0].len() + ind_y] == ind_x * matrix[0].len() + ind_y {
                                tree[ind_x * matrix[0].len() + ind_y] = tree[i * matrix[0].len() + j];
                            }
                            else {
                                let mut current_node = ind_x * matrix[0].len() + ind_y;
                                loop {
                                    let next_node = tree[current_node];
                                    tree[current_node] = tree[i * matrix[0].len() + j];
                                    if next_node == current_node {
                                        break;
                                    }
                                    else {
                                        current_node = next_node;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut count: Vec<u128> = vec![0; matrix[0].len() * matrix.len()];
    let mut result: u128 = 1;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != 9 {
                let mut node = i * matrix[0].len() + j;
                while tree[node] != node {
                    node = tree[node];
                } 
                count[node] += 1;
            }
        }
    }
    count.sort_by(|a, b| b.cmp(a));
    for i in 0..3 {
        if count[i] != 0 {
            result *= count[i];
        }
    }
    result
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
