#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let bit = 25;
    let mut rem = vec![];
    let mut cnt = vec![];
    for i in 0..bit {
        rem.push(vec![0; 1 << i]);
        cnt.push(vec![0; 1 << i]);
    }
    let mut ans = 0;
    for a in a {
        for i in 0..bit {
            rem[i][a % (1 << i)] += a;
            cnt[i][a % (1 << i)] += 1;
        }
        let mut t_cnt = 0;
        let mut t_rem = 0;
        for i in (0..bit).rev() {
            let idx = ((1 << i) - (a % (1 << i))) % (1 << i);
            if cnt[i][idx] - t_cnt != 0 {
                ans += ((rem[i][idx] - t_rem) + (a * (cnt[i][idx] - t_cnt))) / (1 << i);
            }
            t_cnt = cnt[i][idx];
            t_rem = rem[i][idx];
        }
    }
    println!("{:?}", ans);
}
