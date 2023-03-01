use proconio::input;

fn main() {
    input! {
        n:usize,
        mut x:[usize;5*n],
    }
    x.sort();
    let sum = (n..4 * n).into_iter().fold(0, |s, e| s + x[e]);
    println!("{}", sum as f64 / (3 * n) as f64);
}
