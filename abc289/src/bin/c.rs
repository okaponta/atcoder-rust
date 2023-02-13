use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut g = vec![vec![false; n]; m];
    for i in 0..m {
        input! {
            c: usize,
            a: [Usize1;c],
        }
        for j in a {
            g[i][j] = true;
        }
    }
    let mut ans = 0;
    for i in 0..1 << m {
        let mut tmp = vec![false; n];
        for j in 0..m {
            if i >> j & 1 == 1 {
                for k in 0..n {
                    tmp[k] |= g[j][k];
                }
            }
        }
        if tmp.iter().all(|&b| b) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
