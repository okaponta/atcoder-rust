use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[u8;n],
    }
    let mut zero_combo = 0;
    let mut one_combo = 0;
    let mut max_zero_combo = 0;
    let mut max_one_combo = 0;
    for ai in a {
        if ai == 0 {
            zero_combo += 1;
            one_combo -= 1;
        } else {
            one_combo += 1;
            zero_combo -= 1;
        }
        if zero_combo < 0 {
            zero_combo = 0;
        }
        if one_combo < 0 {
            one_combo = 0;
        }
        max_zero_combo = max_zero_combo.max(zero_combo);
        max_one_combo = max_one_combo.max(one_combo);
    }
    println!("{}", max_zero_combo + max_one_combo + 1);
}
