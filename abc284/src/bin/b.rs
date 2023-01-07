use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        input! {
            n:usize,
            a:[usize;n],
        }
        println!("{}", a.into_iter().filter(|i| i % 2 == 1).count());
    }
}
