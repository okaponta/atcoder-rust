use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let ans = a.into_iter().map(|i| i.trailing_zeros()).min().unwrap_or(0);
    println!("{}", ans);
}
