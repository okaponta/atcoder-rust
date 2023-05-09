use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n:i64,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut ans = Vec::new();
    while n.abs() > 0 {
        let b = n.abs() % 2;
        ans.push(b);
        if b == 1 {
            n -= 1;
        }
        n /= -2;
    }
    println!("{}", ans.into_iter().rev().join(""));
}
