use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", f(n));
}

fn f(k: usize) -> usize {
    if k == 0 {
        1
    } else {
        k * f(k - 1)
    }
}
