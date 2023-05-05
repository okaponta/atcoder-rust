use proconio::input;

fn main() {
    input! {
        n:usize,
        abc:[usize;3],
        l:[usize;n],
    }
    println!("{}", dfs(n, 0, &abc, &l, &mut vec![0; 4], &mut vec![0; 4]));
}

fn dfs(
    n: usize,
    i: usize,
    exp: &Vec<usize>,
    l: &Vec<usize>,
    act: &mut Vec<usize>,
    cnt: &mut Vec<usize>,
) -> usize {
    if i == n {
        let mut res = 0;
        for j in 0..3 {
            if cnt[j] == 0 {
                return 1 << 60;
            }
            res += (cnt[j] - 1) * 10;
            res += abs(exp[j], act[j]);
        }
        return res;
    }
    let mut res = 1 << 60;
    for j in 0..4 {
        act[j] += l[i];
        cnt[j] += 1;
        res = res.min(dfs(n, i + 1, exp, l, act, cnt));
        act[j] -= l[i];
        cnt[j] -= 1;
    }
    res
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
