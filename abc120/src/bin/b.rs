use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a:usize,
        b:usize,
        k:Usize1,
    }
    println!(
        "{}",
        (1..101)
            .into_iter()
            .filter(|x| a % x == 0 && b % x == 0)
            .rev()
            .nth(k)
            .unwrap()
    );
}
