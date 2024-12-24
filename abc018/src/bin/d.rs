#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        p:usize,
        q:usize,
        r:usize,
        xyz:[(Usize1,Usize1,usize);r],
    }
    let mut g = vec![vec![]; n];
    for (x, y, z) in xyz {
        g[x].push((y, z));
    }
    let mut ans = 0;
    let mut comb = (1 << p) - 1;
    while comb < 1 << n {
        let mut score = vec![0; m];
        for i in 0..n {
            if comb >> i & 1 == 1 {
                for &(y, z) in &g[i] {
                    score[y] += z;
                }
            }
        }
        score.sort();
        let tmp = (0..q).into_iter().map(|i| score[m - 1 - i]).sum::<usize>();
        ans = ans.max(tmp);
        let x = comb & -comb;
        let y = comb + x;
        comb = ((comb & !y) / x >> 1) | y;
    }
    println!("{}", ans);
}
