use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let mut ans = (n / k).pow(3);
    if k % 2 == 0 {
        ans += ((n + k / 2) / k).pow(3);
    }
    println!("{}", ans);
}
