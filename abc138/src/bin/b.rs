use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[f64;n],
    }
    let ans = 1.0 / a.iter().fold(0.0, |acc, x| acc + 1.0 / *x);
    println!("{}", ans);
}
