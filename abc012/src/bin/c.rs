use proconio::*;

fn main() {
    input! {
        n:usize,
    }
    let rem = 2025 - n;
    for i in 1..=9 {
        if rem % i == 0 && rem / i <= 9 {
            println!("{} x {}", i, rem / i);
        }
    }
}
