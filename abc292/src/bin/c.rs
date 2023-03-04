use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut div = vec![1i64; n];
    for i in 2..n {
        for j in (i..n).step_by(i) {
            div[j] += 1;
        }
    }
    let ans = (1..n).into_iter().fold(0, |s, i| s + div[i] * div[n - i]);
    println!("{}", ans);
}
