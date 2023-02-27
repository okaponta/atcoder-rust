use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
        lr:[(usize,usize);m],
        pq:[(usize,usize);q],
    }
    let mut v = vec![];
    for i in 0..m {
        v.push((0, lr[i].0, lr[i].1, i));
    }
    for i in 0..q {
        v.push((1, pq[i].0, pq[i].1, i));
    }
    let mut fw = FenwickTree::new(n);
    let mut ans = vec![0; q];
    v.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.cmp(&b.0)));
    for (q, l, r, i) in v {
        if q == 0 {
            fw.add(l, 1);
        } else {
            ans[i] = fw.range(l, r);
        }
    }
    for a in ans {
        println!("{}", a);
    }
}

// フェニック木。以下2つができる。1-indexedなので注意
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    pub fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }

    // ai+...+ajを計算する
    pub fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }

    // 和がs以下の位置を返却
    pub fn lower(&self, s: i64) -> usize {
        let mut lower = 0;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
