fn main() {
    const XMIN: i32 = 144;
    const XMAX: i32 = 178;
    const YMIN: i32 = -100;
    const YMAX: i32 = -76;
    let mut max_height = YMIN - 1;
    for x in 1..=178 {
        let mut target_step: Vec<i32> = Vec::new();
        let mut step = 1;
        let mut stopped_in_range = false;
        while step * x - step * (step - 1) / 2 < XMIN {
            if step == x {
                break;
            }
            step += 1;
        }
        if step * x - step * (step - 1) / 2 < XMIN {
            continue;
        }
        while step * x - step * (step - 1) / 2 <= XMAX {
            target_step.push(step);
            if step == x {
                stopped_in_range = true;
                break;
            }
            step += 1;
        }        
        if target_step.len() == 0 {
            continue;
        }
        for y in YMIN - 1..-YMIN * 10 {
            let mut touched = false;
            if stopped_in_range {
                let mut step = target_step[0];
                while step * y - step * (step - 1) / 2 >= YMIN {
                    let target_y = step * y - step * (step - 1) / 2;
                    if (target_y >= YMIN) && (target_y <= YMAX) {
                        touched = true;
                        break;
                    }
                    step += 1;
                }
            }
            else {
                for step in &target_step {
                    let target_y = step * y - step * (step - 1) / 2;
                    if (target_y >= YMIN) && (target_y <= YMAX) {
                        touched = true;
                        break;
                    }
                }
            }
            if touched {
                let mut old_y = 0; let mut s = 1;
                while y * s - s * (s - 1) / 2 > old_y {
                    old_y = y * s - s * (s - 1) / 2; s += 1;
                }
                if old_y > max_height {
                    max_height = old_y;
                }
            }
        }
    }
    println!("{}", max_height);
}
