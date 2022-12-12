use std::fs;

fn input(path: &str) -> String {
    fs::read_to_string(path).expect("Can't read file")
}

fn process(data: String) -> i32 {
    let lines = data.split('\n');
    let mut current_oxygen: Vec<&str> = lines.collect();
    let mut current_c_o2 = current_oxygen.clone();
    for i in 0..12 {
        let (mut count_oxgen_bit, mut count_c_o2_bit) = (0, 0);
        let mut one_bit = Vec::new(); let mut zero_bit = Vec::new();
        if current_c_o2.len() > 1 {
            for item in current_c_o2 {
                if item.chars().nth(i).unwrap() == '1' {
                    count_c_o2_bit += 1;
                    one_bit.push(item);
                }
                else {
                    count_c_o2_bit -= 1;
                    zero_bit.push(item);
                }
            }
            if count_c_o2_bit >= 0 {
                current_c_o2 = zero_bit.clone();
            }
            else {
                current_c_o2 = one_bit.clone();
            }
        }
        if current_oxygen.len() > 1 {
            one_bit = Vec::new(); zero_bit = Vec::new();
            for item in current_oxygen {
                if item.chars().nth(i).unwrap() == '1' {
                    count_oxgen_bit += 1;
                    one_bit.push(item);
                }
                else {
                    count_oxgen_bit -= 1;
                    zero_bit.push(item);
                }
            }
            if count_oxgen_bit >= 0 {
                current_oxygen = one_bit.clone();
            }
            else {
                current_oxygen = zero_bit.clone();
            }
        }
    }
    let (mut oxygen, mut c_o2) = (0, 0);
    const BASE: i32 = 2;
    for i in 0..12 {
        let value: u32 = 12 - (i as u32) - 1;
        if current_oxygen[0].chars().nth(i).unwrap() == '1' {
            oxygen += BASE.pow(value);
        }
        if current_c_o2[0].chars().nth(i).unwrap() == '1' {
            c_o2 += BASE.pow(value);
        }
    }
    oxygen * c_o2
}

fn main() {
    let data = input("input.txt");
    println!("{}", process(data));
}