use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let set = a.into_iter().collect::<HashSet<usize>>();
    let mut remain = n as i64;
    let mut next = 1;
    loop {
        if set.contains(&next) {
            remain -= 1;
        } else {
            remain -= 2;
        }
        if remain < 0 {
            break;
        }
        next += 1;
    }
    println!("{}", next - 1);
}
