use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        c:[[usize;3];3],
    }
    let used = vec![vec![0; 3]; 3];
    let mut q = VecDeque::new();
    q.push_back((used, 0));
    let mut kai = vec![1];
    for i in 0..9 {
        kai.push(kai[i] * (i + 1));
    }
    let mut ans = 0;
    while let Some((v, used)) = q.pop_front() {
        if v.iter().all(|i| i.iter().all(|j| j != &0)) {
            // 操作終了
            continue;
        }
        for i in 0..3 {
            for j in 0..3 {
                if v[i][j] != 0 {
                    continue;
                }
                let mut n = v.clone();
                n[i][j] = c[i][j];
                let mut check = vec![];
                check.push(vec![n[0][0], n[0][1], n[0][2]]);
                check.push(vec![n[1][0], n[1][1], n[1][2]]);
                check.push(vec![n[2][0], n[2][1], n[2][2]]);
                check.push(vec![n[0][0], n[1][0], n[2][0]]);
                check.push(vec![n[0][1], n[1][1], n[2][1]]);
                check.push(vec![n[0][2], n[1][2], n[2][2]]);
                check.push(vec![n[0][0], n[1][1], n[2][2]]);
                check.push(vec![n[0][2], n[1][1], n[2][0]]);
                let mut flg = false;
                for mut c in check {
                    c.sort();
                    if c[0] == 0 && c[1] == c[2] && c[1] != 0 {
                        flg = true;
                        ans += kai[9 - used - 1];
                        break;
                    }
                }
                if !flg {
                    q.push_back((n, used + 1));
                }
            }
        }
    }
    println!("{}", (kai[9] - ans) as f64 / kai[9] as f64);
}
