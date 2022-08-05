use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
    }
    println!("{}", div_ceil(n, 2 * d + 1));
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}
