use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:Usize1,
        k:Usize1,
    }
    println!("{}", (n + k - 1) / k);
}
