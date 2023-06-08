use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        c:usize,
        d:[[usize;c];c],
        col:[[Usize1;n];n]
    }
    let mut counts = vec![vec![0; c]; 3];
    for (i, j) in iproduct!(0..n, 0..n) {
        counts[(i + j) % 3][col[i][j]] += 1;
    }
    let mut ans = 1 << 60;
    for v in (0..c).permutations(3) {
        let mut cost = 0;
        for (i, j) in iproduct!(0..3, 0..c) {
            cost += counts[i][j] * d[j][v[i]];
        }
        ans = ans.min(cost);
    }
    println!("{}", ans);
}
