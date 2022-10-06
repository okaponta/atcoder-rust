use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        v:[i64;n],
    }
    let mut ans = 0;
    // 左から取り出す数
    for i in 0..=n {
        // 右から取り出す数
        for j in 0..=n - i {
            if k < i + j {
                break;
            }
            let tmp = solve(
                i + j,
                k,
                v[0..i].into_iter().collect::<Vec<_>>(),
                v[n - j..n].into_iter().collect::<Vec<_>>(),
            );
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}

fn solve(n: usize, k: usize, left: Vec<&i64>, right: Vec<&i64>) -> i64 {
    let mut unite = left
        .into_iter()
        .chain(right.into_iter())
        .collect::<Vec<_>>();
    unite.sort();
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + unite[i]);
    }
    let mut ans = 0;
    for i in 0..(k + 1 - n).min(n) {
        ans = ans.max(s[n] - s[i]);
    }
    ans
}
