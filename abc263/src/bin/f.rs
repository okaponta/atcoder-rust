use num::pow;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let m = pow(2usize, n);
    input! {
        c: [[usize;n];m],
    }
    let mut dp = vec![0; m];
    for i in 1..n {
        let mut next = vec![0; m];
        let mut seg = SegmentTree::new(m, 0, |a, b| a.max(b));
        for j in 0..m {
            next[j] = dp[j];
            seg.update_tmp(j, dp[j] + c[j][i - 1]);
        }
        seg.update_all();
        for j in 0..m {
            let base = (j >> (i + 1)) << (i + 1);
            let unit = 1 << i;
            let other = if j >> i & 1 == 1 {
                seg.query(base..base + unit)
            } else {
                seg.query(base + unit..base + 2 * unit)
            };
            next[j] += other;
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 0..m {
        let tmp = dp[i] + c[i][n - 1];
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}

pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let m = size.next_power_of_two();
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, mut k: usize, value: T) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    pub fn update_tmp(&mut self, k: usize, value: T) {
        self.seg[k + self.n - 1] = value;
    }

    pub fn update_all(&mut self) {
        for i in (0..self.n - 1).rev() {
            self.seg[i] = (self.f)(self.seg[2 * i + 1], self.seg[2 * i + 2]);
        }
    }

    // 半開区間なので注意
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> T {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            self.initial_value
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            (self.f)(x, y)
        }
    }
}
