use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
        c:[u128;n],
    }
    let all = (0..n).into_iter().map(|i| c[i]).sum::<u128>();
    let mut dp = vec![0; n];
    let mut dp2 = vec![0; n];
    let mut dp3 = vec![0; n];
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    dfs1(0, 0, &g, &mut dp, &mut dp2, &c);
    dfs2(0, 0, &g, &dp, &dp2, &mut dp3, &c, all, 0);
    let mut ans = 1 << 100;
    for i in 0..n {
        ans = ans.min(dp[i] + dp3[i]);
    }
    println!("{}", ans);
}

// 配下
fn dfs1(
    cur: usize,
    prev: usize,
    g: &Vec<Vec<usize>>,
    dp: &mut Vec<u128>,
    dp2: &mut Vec<u128>,
    c: &Vec<u128>,
) -> u128 {
    let mut tmp = 0;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        tmp += dfs1(next, cur, g, dp, dp2, c);
        dp[cur] += dp[next];
    }
    dp[cur] += tmp;
    dp2[cur] = tmp + c[cur];
    tmp + c[cur]
}

// その他
fn dfs2(
    cur: usize,
    prev: usize,
    g: &Vec<Vec<usize>>,
    dp: &Vec<u128>,
    dp2: &Vec<u128>,
    dp3: &mut Vec<u128>,
    c: &Vec<u128>,
    all: u128,
    base: u128, // 今までの合計
) {
    dp3[cur] = base;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        let tmp = base + dp[cur] - dp[next] - dp2[next] + all - dp2[next];
        dfs2(next, cur, g, dp, dp2, dp3, c, all, tmp);
    }
}
