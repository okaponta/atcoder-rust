use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        p:[Usize1; n],
    }
    let a = p
        .iter()
        .enumerate()
        .map(|(i, p)| (p + i) as i64)
        .collect::<Vec<_>>();
    let b = p
        .iter()
        .enumerate()
        .map(|(i, p)| *p as i64 - i as i64)
        .collect::<Vec<_>>();

    let inf = 1 << 60;
    let mut ans = vec![inf; n];

    // j<i, p[j]<p[i]
    let mut seg = SegmentTree::new(n, -inf, |a, b| a.max(b));
    for i in 0..n {
        // a[i] - max a[j]
        ans[i] = ans[i].min(a[i] - seg.query(0..p[i]));
        seg.update(p[i], a[i]);
    }

    // j<i, p[i]<p[j]
    let mut seg = SegmentTree::new(n, inf, |a, b| a.min(b));
    for i in 0..n {
        // min b[j] - b[i]
        ans[i] = ans[i].min(seg.query(p[i]..n) - b[i]);
        seg.update(p[i], b[i]);
    }

    // i<j, p[j]<p[i]
    let mut seg = SegmentTree::new(n, -inf, |a, b| a.max(b));
    for i in (0..n).rev() {
        // b[i] - max b[j]
        ans[i] = ans[i].min(b[i] - seg.query(0..p[i]));
        seg.update(p[i], b[i]);
    }

    // i<j, p[i]<p[j]
    let mut seg = SegmentTree::new(n, inf, |a, b| a.min(b));
    for i in (0..n).rev() {
        // min a[j] - a[i]
        ans[i] = ans[i].min(seg.query(p[i]..n) - a[i]);
        seg.update(p[i], a[i]);
    }

    println!("{}", ans.iter().join(" "));
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
