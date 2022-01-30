use proconio::input;

fn main() {
    input! {
        n:i64,
    }
    let max = 1 << 31;
    println!("{}", if -max <= n && n < max { "Yes" } else { "No" });
}
