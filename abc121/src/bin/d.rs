use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a:Usize1,
        b:usize,
    }
    println!("{}", sxor(b) ^ sxor(a));
}

fn sxor(a: usize) -> usize {
    if a % 2 != 0 {
        return (a / 2 + 1) % 2;
    }
    a + (a / 2) % 2
}
