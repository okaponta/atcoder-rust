#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut s1 = vec![0];
    let mut s2 = vec![0];
    for i in 0..n {
        if i % 2 == 0 {
            s1.push(s1[i] + a[i]);
            s2.push(s2[i]);
        } else {
            s1.push(s1[i]);
            s2.push(s2[i] + a[i]);
        }
    }
    let mut ans = -1 << 30;
    for i in 0..n {
        let mut tmp1 = -1 << 30;
        let mut tmp2 = -1 << 30;
        for j in 0..n {
            if i == j {
                continue;
            }
            if i.min(j) % 2 == 0 {
                let ao = s2[i.max(j) + 1] - s2[i.min(j)];
                let ta = s1[i.max(j) + 1] - s1[i.min(j)];
                if tmp2 < ao {
                    tmp1 = ta;
                    tmp2 = ao;
                }
            } else {
                let ao = s1[i.max(j) + 1] - s1[i.min(j)];
                let ta = s2[i.max(j) + 1] - s2[i.min(j)];
                if tmp2 < ao {
                    tmp1 = ta;
                    tmp2 = ao;
                }
            }
        }
        ans = ans.max(tmp1);
    }
    println!("{}", ans);
}
