use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        k:usize,
        p:[Usize1;n],
    }
    if k == 0 {
        println!("{}", p.iter().map(|i| i + 1).join(" "));
        return;
    }
    let mut rev_index = vec![0; n];
    for i in 0..n {
        rev_index[p[i]] = i;
    }
    let ans1 = calcans(n, k, 0, &p, &rev_index);

    let mut first = 0;
    loop {
        if n - rev_index[first] <= k {
            break;
        }
        first += 1;
    }
    let mut rot = p.clone();
    rot.rotate_right(n - rev_index[first]);
    let mut rev_rot = vec![0; n];
    for i in 0..n {
        rev_rot[rot[i]] = i;
    }
    let ans2 = calcans(
        n,
        k + rev_index[first] - n,
        n - rev_index[first],
        &rot,
        &rev_rot,
    );
    if ans1 < ans2 {
        println!("{}", ans1.iter().join(" "));
    } else {
        println!("{}", ans2.iter().join(" "));
    }
}

fn calcans(
    n: usize,
    mut k: usize,
    zero_cost: usize,
    p: &Vec<usize>,
    rev_index: &Vec<usize>,
) -> Vec<usize> {
    let mut seg = SegmentTree::new(n, 1_000_000, |a, b| a.min(b));
    for i in 0..n {
        seg.update_tmp(i, p[i]);
    }
    seg.update_all();
    let mut index = 0;
    let mut tmp = vec![];
    while index < n {
        let next = seg.query(index..index.max(zero_cost) + k + 1);
        tmp.push(next);
        k -= rev_index[next].max(zero_cost) - index.max(zero_cost);
        index = rev_index[next] + 1;
    }
    let mut ans = vec![];
    for i in 0..tmp.len() - k {
        ans.push(tmp[i] + 1);
    }
    ans
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
