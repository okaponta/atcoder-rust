use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[[usize;n];n],
    }
    // 半分列挙する
    let mut q = VecDeque::new();
    // 縦から何こめ、横から何こめ、XOR
    q.push_back((0, 0, a[0][0]));
    for _ in 2..n {
        let mut nextq = VecDeque::new();
        while let Some((i, j, x)) = q.pop_front() {
            nextq.push_back((i + 1, j, x ^ a[i + 1][j]));
            nextq.push_back((i, j + 1, x ^ a[i][j + 1]));
        }
        q = nextq;
    }
    // もう半分の列挙
    let mut q2 = VecDeque::new();
    // 縦から何こめ、横から何こめ、XOR
    q2.push_back((n - 1, n - 1, a[n - 1][n - 1]));
    for _ in 1..n {
        let mut nextq2 = VecDeque::new();
        while let Some((i, j, x)) = q2.pop_front() {
            nextq2.push_back((i - 1, j, x ^ a[i - 1][j]));
            nextq2.push_back((i, j - 1, x ^ a[i][j - 1]));
        }
        q2 = nextq2;
    }
    let mut map = HashMap::new();
    for (i, _, x) in q2 {
        *map.entry((i, x)).or_insert(0usize) += 1;
    }
    let mut ans = 0;
    for (i, _, x) in q {
        if let Some(c) = map.get(&(i, x)) {
            ans += c;
        }
        if let Some(c) = map.get(&(i + 1, x)) {
            ans += c;
        }
    }
    println!("{}", ans);
}
