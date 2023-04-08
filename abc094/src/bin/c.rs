use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        x:[usize;n],
    }
    let mut y = x.clone();
    y.sort();
    let a = y[n / 2 - 1];
    let b = y[n / 2];
    for x in x {
        println!("{}", if x <= a { b } else { a });
    }
}
