#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        s:Chars,
    }
    let mut seg = SegmentTree::new(n, (0, '#', 0, '#', 0, 1), merge);
    for i in 0..n {
        seg.update_tmp(i, (1, s[i], 1, s[i], 1, 1));
    }
    seg.update_all();
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {i:Usize1, x:char}
            seg.update(i, (1, x, 1, x, 1, 1));
        } else {
            input! {l:Usize1, r:usize}
            println!("{}", seg.query(l..r).4);
        }
    }
}

fn merge(
    a: (usize, char, usize, char, usize, usize),
    b: (usize, char, usize, char, usize, usize),
) -> (usize, char, usize, char, usize, usize) {
    if a.3 != b.1 {
        // マージなし
        return (a.0, a.1, b.2, b.3, a.4.max(b.4), a.5 + b.5);
    }
    if a.5 == a.0 && b.5 == b.0 {
        // 全一致
        return (a.0 + b.0, a.1, a.2 + b.2, b.3, a.4 + b.4, a.5 + b.5);
    }
    if a.5 == a.0 {
        // aが全部同じ
        return (a.0 + b.0, a.1, b.2, b.3, (a.0 + b.0).max(b.4), a.5 + b.5);
    }
    if b.5 == b.0 {
        // bが全部同じ
        return (a.0, a.1, a.2 + b.2, b.3, (a.2 + b.2).max(a.4), a.5 + b.5);
    }
    // 普通のマージ
    (a.0, a.1, b.2, b.3, a.4.max(b.4).max(a.2 + b.0), a.5 + b.5)
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

    // 半開区間なので注意
    pub fn get(&self, k: usize) -> T {
        self.query(k..k + 1)
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
