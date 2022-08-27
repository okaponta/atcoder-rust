use proconio::input;

fn main() {
    input! {
        n:usize,
        w:[i64;n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + w[i];
    }
    let sum = s[n];
    println!("{}", s.iter().map(|i| (sum - 2 * i).abs()).min().unwrap());
}
