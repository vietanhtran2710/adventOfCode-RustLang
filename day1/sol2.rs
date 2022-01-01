use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong reading the file");
    let (mut x, mut y, mut z) = (0, 0, 0); let mut count = 0;
    let lines = contents.split('\n');
    for (index, line) in lines.enumerate() {
        let value = line.parse::<i32>().unwrap();
        if index == 0 { x = value; }
        else if index == 1 { y = value; }
        else if index == 2 {
                z = value;
            }
        else {
            if value > x {
                count += 1;
            }
            x = y; y = z; z = value;
        }
    }
    println!("{}", count);
}