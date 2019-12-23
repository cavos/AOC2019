use std::char;

pub fn solve(pass_from: u32, pass_until: u32) {
    let mut password_count = 0u32;
    // let mut final_password = String::new();
    let mut password_count_2 = 0u32;
    for password in pass_from..pass_until + 1 {
        let pswd: Vec<char> = password.to_string().chars().collect();
        let is_increasing = pswd.windows(2).all(|x| x[0] <= x[1]);
        if is_increasing {
            let has_duplicate = pswd.windows(2).any(|x| x[0] == x[1]);
            if has_duplicate {
                password_count += 1;

                let mut add_crit_pass = false;
                for d in 0..10 {
                    let cnt = pswd
                        .iter()
                        .filter(|&&x| x == char::from_digit(d, 10).unwrap())
                        .count();
                    add_crit_pass |= cnt == 2;
                }
                if add_crit_pass {
                    password_count_2 += 1;
                }
            }
        }
    }

    println!("Day 04.1: matching password count: {:}", password_count);
    println!("Day 04.2: matching password count: {}", password_count_2);
}
