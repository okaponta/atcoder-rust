#[rustfmt::skip]
use {proconio::{marker::*, *}, std::*};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {
        mut h:usize,
        mut w:usize,
        mut s:[Chars;h],
    }
    if w < h {
        s = rotate(h, w, s);
        mem::swap(&mut h, &mut w);
    }
    let mut ans = 0usize;
    let n = h * w;
    let mut cnt = vec![0; 2 * n + 1];
    for i in 0..h {
        let mut dp = vec![0; w];
        for j in i..h {
            for k in 0..w {
                dp[k] += if s[j][k] == '.' { 1 } else { -1 };
            }
            let mut sum = vec![0; w + 1];
            sum[0] = n;
            for k in 0..w {
                sum[k + 1] = add(sum[k], dp[k]);
                cnt[sum[k]] += 1;
                ans += cnt[sum[k + 1]];
            }
            for k in 0..w {
                cnt[sum[k]] = 0;
            }
        }
    }
    println!("{}", ans);
}

fn add(a: usize, b: i32) -> usize {
    if 0 < b {
        a + b as usize
    } else {
        a - (-b as usize)
    }
}

fn rotate(h: usize, w: usize, s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; h]; w];
    for i in 0..h {
        for j in 0..w {
            res[j][i] = s[i][j];
        }
    }
    res
}
