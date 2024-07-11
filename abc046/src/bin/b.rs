use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let mut ans = k;
    for _ in 1..n {
        ans *= k - 1;
    }
    println!("{}", ans);
}
