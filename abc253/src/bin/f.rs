use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
    }
    let mut query = vec![];
    let mut special = vec![q; n + 1];
    // key: index,val:出力するjの数
    let mut map = HashMap::new();
    for idx in 0..q {
        input! {qq: u8}
        if qq == 1 {
            input! {l:usize,r:usize,x:i64}
            query.push((idx, qq, l, r, x));
        } else if qq == 2 {
            input! {i:usize, x:i64}
            query.push((idx, qq, i, 0, x));
            special[i] = idx;
        } else {
            input! {i:usize, j:usize}
            query.push((idx, qq, i, j, 0));
            if special[i] != q {
                map.entry(special[i]).or_insert(vec![]).push(j);
            }
        }
    }

    let mut fw = FenwickTree::new(m);
    let mut submap = HashMap::new();
    let mut base = vec![0; n + 1];
    for (idx, qq, i, j, x) in query {
        if qq == 1 {
            fw.add(i, x);
            if j != m {
                fw.add(j + 1, -x);
            }
        } else if qq == 2 {
            if map.contains_key(&idx) {
                for &j in map.get(&idx).unwrap() {
                    submap.insert((i, j), fw.sum(j));
                }
            }
            base[i] = x;
        } else {
            let sub = *submap.get(&(i, j)).unwrap_or(&0);
            let ans = base[i] + fw.sum(j) - sub;
            println!("{}", ans);
        }
    }
}

// フェニック木。以下2つができる
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }
}
