use proconio::input;

fn main() {
    input! {
        _n:usize,
        x:usize,
        y:usize,
        z:usize,
    }
    let ans = x.min(y) < z && z < x.max(y);
    println!("{}", if ans { "Yes" } else { "No" });
}
