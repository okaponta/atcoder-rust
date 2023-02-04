use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(i64,i64);n],
    }
    for (a, b) in ab {
        println!("{}", a + b);
    }
}
