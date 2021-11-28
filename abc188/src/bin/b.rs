use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[i64;n],
       b:[i64;n],
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += a[i] * b[i];
    }
    println!("{}", if ans == 0 { "Yes" } else { "No" });
}
