use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_vec: Vec<char> = s.chars().collect();

    // 条件1の判定
    let mut b_positions = Vec::new();
    for (i, &c) in s_vec.iter().enumerate() {
        if c == 'B' {
            b_positions.push(i);
        }
    }
    for i in 0..b_positions.len() {
        for j in i + 1..b_positions.len() {
            if (b_positions[i] % 2 == 0 && b_positions[j] % 2 == 1)
                || (b_positions[i] % 2 == 1 && b_positions[j] % 2 == 0)
            {
                // 条件1を満たす
            } else {
                println!("No");
                return;
            }
        }
    }

    // 条件2の判定
    let k_pos = s_vec.iter().position(|&c| c == 'K').unwrap();
    let mut r_positions = Vec::new();
    for (i, &c) in s_vec.iter().enumerate() {
        if c == 'R' {
            r_positions.push(i);
        }
    }
    let mut flag = false;
    for i in 0..r_positions.len() {
        for j in i + 1..r_positions.len() {
            if r_positions[i] < k_pos && k_pos < r_positions[j] {
                flag = true;
            }
        }
    }
    if flag {
        // 条件2を満たす
    } else {
        println!("No");
        return;
    }

    println!("Yes");
}
