use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a:Usize1,
        b:usize,
    }
    println!("{}", s(b) ^ s(a));
}

fn s(a: usize) -> usize {
    if a % 2 != 0 {
        (a / 2 + 1) % 2
    } else {
        a + (a / 2) % 2
    }
}
