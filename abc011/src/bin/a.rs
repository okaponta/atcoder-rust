use proconio::*;

fn main() {
    input! {
        mut n:usize,
    }
    n %= 12;
    println!("{}", n + 1);
}
