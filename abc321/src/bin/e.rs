use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:usize,
        nxk:[(usize,usize,usize);t],
    }
    let mut pow2 = vec![1usize];
    for i in 0..61 {
        pow2.push(pow2[i] * 2);
    }
    for (n, x, k) in nxk {
        let mut ans = 0;
        ans += count_below(n, x, k, &pow2);
        if x != 1 && 0 < k {
            ans += dfs(x >> 1, x, n, k - 1, &pow2);
        }
        println!("{}", ans);
    }
}

fn count_below(n: usize, x: usize, k: usize, pow2: &Vec<usize>) -> usize {
    if 60 < x.next_power_of_two().trailing_zeros() + k as u32 {
        return 0;
    }
    ((x << k) + pow2[k] - 1).min(n).saturating_sub((x << k) - 1)
}

fn dfs(cur: usize, prev: usize, n: usize, k: usize, pow2: &Vec<usize>) -> usize {
    let mut res = 0;
    if k == 0 {
        return 1;
    }
    let below2 = (cur << 2) + 1 - prev;
    res += count_below(n, below2, k - 1, pow2);
    if cur != 1 {
        res += dfs(cur >> 1, cur, n, k - 1, pow2);
    }
    res
}
