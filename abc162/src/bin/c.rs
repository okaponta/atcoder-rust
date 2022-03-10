use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let mut g = vec![vec![0; k + 1]; k + 1];
    for (a, b) in iproduct!(1..=k, 1..=k) {
        g[a][b] = gcd(a, b);
    }
    let mut ans = 0;
    for (a, b, c) in iproduct!(1..=k, 1..=k, 1..=k) {
        ans += g[g[a][b]][c];
    }
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
