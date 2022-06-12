use proconio::input;

fn main() {
    input! {
        n:usize,
        b:[usize;n-1],
    }
    let mut ans = b[0] + b[n - 2];
    for i in 1..n - 1 {
        ans += b[i - 1].min(b[i]);
    }
    println!("{}", ans);
}
