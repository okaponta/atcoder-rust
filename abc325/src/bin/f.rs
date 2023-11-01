use proconio::input;

fn main() {
    input! {
        n:usize,
        d:[usize;n],
        l1:usize,
        c1:usize,
        k1:usize,
        l2:usize,
        c2:usize,
        k2:usize,
    }
    let mut dp = vec![vec![(0, 0); k2 + 1]; k1 + 1];
    let mut ans = 1 << 60;
    for i in 0..=k1 {
        for j in 0..=k2 {
            if i < k1 {
                dp[i + 1][j] = dp[i + 1][j].max(add(dp[i][j], l1, &d));
            }
            if j < k2 {
                dp[i][j + 1] = dp[i][j + 1].max(add(dp[i][j], l2, &d));
            }
            if dp[i][j].0 == n {
                ans = ans.min(i * c1 + j * c2);
            }
        }
    }
    if ans == 1 << 60 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}

fn add(cur: (usize, usize), l: usize, d: &Vec<usize>) -> (usize, usize) {
    if cur.0 == d.len() {
        return cur;
    }
    let tmp = (cur.0, cur.1 + l);
    if d[tmp.0] <= tmp.1 {
        return (tmp.0 + 1, 0);
    }
    tmp
}
