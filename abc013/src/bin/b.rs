use proconio::*;

fn main() {
    input! {
        a:i32,
        b:i32,
    }
    let ans = (a - b).abs().min(10 + a - b).min(10 + b - a);
    println!("{}", ans);
}
