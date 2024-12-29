use itertools::*;
use proconio::*;

fn main() {
    input! {
        b:[[i64;3];2],
        c:[[i64;2];3],
    }
    let ans = dfs(0, &mut vec![vec![0; 3]; 3], &b, &c);
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn dfs(d: u8, cur: &mut Vec<Vec<u8>>, b: &Vec<Vec<i64>>, c: &Vec<Vec<i64>>) -> (i64, i64) {
    if d == 9 {
        return calc(cur, b, c);
    }
    let mut res = if d % 2 == 0 {
        (0, 1 << 60)
    } else {
        (1 << 60, 0)
    };
    for (i, j) in iproduct!(0..3, 0..3) {
        if cur[i][j] != 0 {
            continue;
        }
        cur[i][j] = (d % 2) + 1;
        let tmp = dfs(d + 1, cur, b, c);
        if d % 2 == 0 {
            if res.0 - res.1 < tmp.0 - tmp.1 {
                res = tmp;
            }
        } else {
            if res.1 - res.0 < tmp.1 - tmp.0 {
                res = tmp;
            }
        }
        cur[i][j] = 0;
    }
    res
}

fn calc(cur: &mut Vec<Vec<u8>>, b: &Vec<Vec<i64>>, c: &Vec<Vec<i64>>) -> (i64, i64) {
    let mut res = (0, 0);
    for (i, j) in iproduct!(0..2, 0..3) {
        if cur[i][j] == cur[i + 1][j] {
            res.0 += b[i][j];
        } else {
            res.1 += b[i][j];
        }
    }
    for (i, j) in iproduct!(0..3, 0..2) {
        if cur[i][j] == cur[i][j + 1] {
            res.0 += c[i][j];
        } else {
            res.1 += c[i][j];
        }
    }
    res
}
