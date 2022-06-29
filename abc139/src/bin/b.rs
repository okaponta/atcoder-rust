use num::Integer;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a:Usize1,
        b:Usize1,
    }
    println!("{}", b.div_ceil(&a));
}
