use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
