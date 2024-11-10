use std::collections::VecDeque;

use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        h:[usize;n],
        lr:[(Usize1,Usize1);q],
    }
    let mut lri = vec![vec![]; n];
    for i in 0..q {
        lri[lr[i].0].push((lr[i].1, i));
    }
    let mut ans = vec![0; q];
    let mut seg = SegmentTree::new(n, 0, |a, b| a + b);
    let mut q = VecDeque::new();
    for i in (0..n).rev() {
        for &(r, i) in &lri[i] {
            ans[i] = seg.query(r + 1..n);
        }
        while let Some((hj, j)) = q.pop_front() {
            if hj < h[i] {
                seg.update(j, 0);
            } else {
                q.push_front((hj, j));
                break;
            }
        }
        q.push_front((h[i], i));
        seg.update(i, 1);
    }
    for ans in ans {
        println!("{ans}");
    }
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
