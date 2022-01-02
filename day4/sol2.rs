use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn get_win_value(board: Vec<Vec<i32>>) -> i32 {
    for i in 0..5 {
        let mut sum = 0;
        for j in 0..5 {
            sum += board[i][j];
        }
        if sum == 0 {
            let mut result = 0;
            for x in 0..5 {
                for y in 0..5 {
                    result += board[x][y];
                }
            }
            return result;
        }
    }
    for j in 0..5 {
        let mut sum = 0;
        for i in 0..5 {
            sum += board[i][j];
        }
        if sum == 0 {
            let mut result = 0;
            for x in 0..5 {
                for y in 0..5 {
                    result += board[x][y];
                }
            }
            return result;
        }
    }
    return 0;
}

fn process(data: String) -> i32 {
    let mut count_board = 0;
    let lines: Vec<&str> = data.split('\n').collect();
    let values: Vec<i32> = lines[0].split(",").map(|s| s.parse().expect("parse error")).collect();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut won: Vec<bool> = Vec::new();
    for i in (2..lines.len()).step_by(6) {
        count_board += 1;
        won.push(false);
        let mut single_board: Vec<Vec<i32>> = Vec::new();
        for j in 0..5 {
            let row: Vec<i32> = lines[i + j].split_whitespace().map(|s| s.parse().expect("parse error")).collect();
            single_board.push(row);
        }
        boards.push(single_board);
    }
    for item in values {
        for (_index, board) in boards.iter_mut().enumerate() {
            if !won[_index] {
                for (_i, row) in board.iter_mut().enumerate() {
                    for (_y, col) in row.iter_mut().enumerate() {
                        if *col == item {
                            *col = 0;
                        }
                    }
                }
                let val = get_win_value(board.to_vec());
                if val != 0 {
                    if count_board == 1 {
                        return val * item;
                    }
                    else {
                        won[_index] = true;
                        count_board -= 1;
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}
