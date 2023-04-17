use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        x:usize,
        a:[usize;n],
    }
    println!("{}", a.into_iter().fold(x + n, |s, i| s + (d - 1) / i));
}
