use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h],
        b:[[usize;w];h],
    }
    for v1 in (0..h).permutations(h) {
        for v2 in (0..w).permutations(w) {
            let mut flg = true;
            for (i, j) in iproduct!(0..h, 0..w) {
                if a[v1[i]][v2[j]] != b[i][j] {
                    flg = false;
                    break;
                }
            }
            if flg {
                let ans = inversion_num(v1) + inversion_num(v2);
                println!("{}", ans);
                return;
            }
        }
    }
    println!("-1");
}

fn inversion_num(a: Vec<usize>) -> i64 {
    let a = a.into_iter().map(|i| i + 1).collect::<Vec<_>>();
    let max = *a.iter().max().unwrap();
    let mut inv = 0;
    let mut fw = FenwickTree::new(max);
    // 注意：1-indexedで行うこと！
    for i in 0..a.len() {
        inv += i as i64 - fw.sum(a[i]);
        fw.add(a[i], 1);
    }
    inv
}

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

    // aiをvで置き換える
    pub fn update(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let cur = self.range(i, i);
        self.add(i, v - cur);
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
