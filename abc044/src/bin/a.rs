use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        x:usize,
        y:usize,
    }
    let ans = if n <= k { x * n } else { x * k + y * (n - k) };
    println!("{}", ans);
}
