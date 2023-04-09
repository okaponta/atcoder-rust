use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        mut a:usize,
        mut b:usize,
    }
    if a < b {
        swap(&mut a, &mut b);
    }
    let mut ans = 0;
    while a != b {
        ans += a / b;
        a = a % b;
        if a == 0 {
            break;
        } else {
            swap(&mut a, &mut b);
        }
    }
    println!("{}", ans.saturating_sub(1));
}
