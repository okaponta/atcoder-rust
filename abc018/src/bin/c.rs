#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        r:usize,
        c:usize,
        k:usize,
        s:[Chars;r],
    }
    let mut cnt = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            if s[i][j] == 'x' {
                cnt[i][j] = k;
            }
        }
    }
    for i in 0..r {
        for j in 1..c {
            if 0 < cnt[i][j - 1] {
                cnt[i][j] = cnt[i][j].max(cnt[i][j - 1] - 1);
            }
        }
    }
    for i in 0..r {
        for j in (0..c - 1).rev() {
            if 0 < cnt[i][j + 1] {
                cnt[i][j] = cnt[i][j].max(cnt[i][j + 1] - 1);
            }
        }
    }
    for j in 0..c {
        for i in 1..r {
            if 0 < cnt[i - 1][j] {
                cnt[i][j] = cnt[i][j].max(cnt[i - 1][j] - 1);
            }
        }
    }
    for j in 0..c {
        for i in (0..r - 1).rev() {
            if 0 < cnt[i + 1][j] {
                cnt[i][j] = cnt[i][j].max(cnt[i + 1][j] - 1);
            }
        }
    }
    let mut ans = 0;
    for i in k - 1..=r - k {
        for j in k - 1..=c - k {
            if cnt[i][j] == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
