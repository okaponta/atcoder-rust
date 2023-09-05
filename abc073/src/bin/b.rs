use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        lr:[(Usize1,usize);n],
    }
    println!("{}", lr.iter().fold(0, |s, (l, r)| s + r - l));
}
