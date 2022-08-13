use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize, w:usize,
        a:[[usize;w];h],
        h2:usize, w2:usize,
        b:[[usize;w2];h2],
    }
    for hv in (0..h).combinations(h2) {
        for wv in (0..w).combinations(w2) {
            let mut new = vec![vec![0; w2]; h2];
            for i in 0..h2 {
                for j in 0..w2 {
                    new[i][j] = a[hv[i]][wv[j]];
                }
            }
            if new == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
