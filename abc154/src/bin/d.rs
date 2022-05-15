use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        p:[usize;n],
    }
    let mut sum = 0;
    for i in 0..k {
        sum += p[i];
    }
    let mut max = sum;
    for i in k..n {
        sum = sum + p[i] - p[i - k];
        max = max.max(sum);
    }
    println!("{}", (max + k) as f64 / 2.0);
}
