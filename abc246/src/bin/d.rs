use proconio::input;

const MAX: i64 = 1_000_000;

fn main() {
    input! {
        n:i64,
    }
    let mut ans = 1 << 60;
    let mut b = MAX;
    for a in 0..MAX {
        while f(a, b) >= n && b >= 0 {
            ans = ans.min(f(a, b));
            b -= 1;
        }
    }
    println!("{}", ans);
}

fn f(a: i64, b: i64) -> i64 {
    (a + b) * (a * a + b * b)
}
