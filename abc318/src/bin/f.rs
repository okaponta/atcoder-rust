use proconio::input;

fn main() {
    input! {
        n:usize,
        x:[i64;n],
        mut l:[i64;n],
    }
    l.sort();
    let mut cand = vec![];
    for i in 0..n {
        for j in 0..n {
            cand.push(x[i] - l[j]);
            cand.push(x[i] + l[j]);
        }
    }
    cand.sort();
    cand.dedup();
    let mut res = vec![false; cand.len()];
    let mut ans = 0;
    for i in 0..cand.len() {
        if is_ok(cand[i], &x, &l, n) {
            ans += 1;
            res[i] = true;
            if i != 0 && res[i - 1] {
                // 間もOKか調べる
                if cand[i] - cand[i - 1] == 1 {
                    continue;
                }
                if is_ok((cand[i] + cand[i - 1]) / 2, &x, &l, n) {
                    ans += cand[i] - cand[i - 1] - 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn is_ok(k: i64, x: &Vec<i64>, l: &Vec<i64>, n: usize) -> bool {
    let mut dist = vec![];
    for i in 0..n {
        dist.push((x[i] - k).abs());
    }
    dist.sort();
    (0..n).into_iter().all(|i| dist[i] <= l[i])
}
