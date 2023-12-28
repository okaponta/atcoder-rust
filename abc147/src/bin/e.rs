use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[i64;w];h],
        b:[[i64;w];h],
    }
    let mut diff = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            diff[i][j] = (a[i][j] - b[i][j]).abs() as usize;
        }
    }
    let mut dp = vec![vec![vec![false; 80 * (h + w)]; w]; h];
    dp[0][0][diff[0][0]] = true;
    for i in 0..h {
        for j in 0..w {
            for k in 0..80 * (h + w) {
                if !dp[i][j][k] {
                    continue;
                }
                if i + 1 < h {
                    dp[i + 1][j][k + diff[i + 1][j]] = true;
                    dp[i + 1][j][abs(k, diff[i + 1][j])] = true;
                }
                if j + 1 < w {
                    dp[i][j + 1][k + diff[i][j + 1]] = true;
                    dp[i][j + 1][abs(k, diff[i][j + 1])] = true;
                }
            }
        }
    }
    let ans = (0..80 * (h + w))
        .into_iter()
        .find(|&k| dp[h - 1][w - 1][k])
        .unwrap();
    println!("{}", ans);
}

fn abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
