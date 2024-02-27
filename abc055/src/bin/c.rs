use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let ans = ((n * 2 + m) / 4).min(m / 2);
    println!("{}", ans);
}
