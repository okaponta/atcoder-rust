use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        d:usize,
    }
    let mut ans = d;
    let sq = (0..2_000_000)
        .into_iter()
        .map(|i| i * i)
        .collect::<Vec<_>>();
    let mut x = 0usize;
    while sq[x.saturating_sub(1)] <= d {
        let a = sq[sq.lower_bound(&(d.saturating_sub(sq[x])))];
        let b = sq[sq.lower_bound(&(d.saturating_sub(sq[x]))).saturating_sub(1)];
        ans = ans.min(abs(sq[x] + a, d)).min(abs(sq[x] + b, d));
        x += 1;
    }
    println!("{}", ans);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
