use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    let mut dp = vec![0; 1 << n];
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..i {
            if ab[i].0 == ab[j].0 || ab[i].1 == ab[j].1 {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    dfs(true, 0, &g, &mut dp, n);
    println!("{}", if dp[0] == 1 { "Takahashi" } else { "Aoki" });
}

fn dfs(is_tak: bool, cur: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<u8>, n: usize) {
    if dp[cur] != 0 {
        return;
    }
    for n1 in 0..n {
        if cur >> n1 & 1 == 1 {
            // 訪問済み
            continue;
        }
        for &n2 in &g[n1] {
            if cur >> n2 & 1 == 1 {
                // 訪問済み
                continue;
            }
            let next = cur + (1 << n1) + (1 << n2);
            if dp[next] == 0 {
                dfs(is_tak, next, g, dp, n);
            }
            if dp[next] == 2 {
                dp[cur] = 1;
                return;
            }
        }
    }
    dp[cur] = 2;
}
