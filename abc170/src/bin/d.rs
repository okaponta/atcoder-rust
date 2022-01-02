use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:i32,a:[i32;n],
    }
    let mut ex = vec![true; 1000001];
    let mut set = HashSet::new();
    for ai in a.clone() {
        if set.contains(&ai) {
            ex[ai as usize] = false;
            continue;
        }
        set.insert(ai);
        let mut mul = ai * 2;
        while mul <= 1000000 {
            ex[mul as usize] = false;
            mul += ai;
        }
    }
    let mut count = 0;
    for ai in a {
        if ex[ai as usize] {
            count += 1;
        }
    }
    println!("{}", count);
}
