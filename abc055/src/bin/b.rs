use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
        ans %= 1_000_000_007;
    }
    println!("{}", ans);
}
