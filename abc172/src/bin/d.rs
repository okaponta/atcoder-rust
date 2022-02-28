use proconio::input;

fn main() {
    input! {
       n:usize,
    }
    let half = n / 2;
    let mut ans = 0;
    for i in 1..=half {
        let mx = n - n % i;
        ans += (i + mx) * (mx / i) / 2;
    }
    ans += (half + 1 + n) * (n - half) / 2;
    println!("{}", ans);
}
