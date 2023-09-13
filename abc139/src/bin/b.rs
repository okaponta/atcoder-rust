use num::Integer;
use proconio::{input, marker::Usize1};

#[allow(unstable_name_collisions)]
fn main() {
    input! {
        a:Usize1,
        b:Usize1,
    }
    println!("{}", b.div_ceil(&a));
}
