use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,
       ab:[(Usize1,Usize1);m],
       cd:[(Usize1,Usize1);m],
    }
    let mut x = vec![vec![false; n]; n];
    let mut y = vec![vec![false; n]; n];
    for (a, b) in ab {
        x[a][b] = true;
        x[b][a] = true;
    }
    for (c, d) in cd {
        y[c][d] = true;
        y[d][c] = true;
    }
    let same = (0..n).permutations(n).any(|p| {
        for (i, j) in iproduct!(0..n, 0..n) {
            if x[i][j] != y[p[i]][p[j]] {
                return false;
            }
        }
        true
    });
    println!("{}", if same { "Yes" } else { "No" });
}
