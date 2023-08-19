use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        x:[i64;n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += x[i].min((x[i] - k).abs());
    }
    println!("{}", ans * 2);
}
