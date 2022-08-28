use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
       n:usize,
       q:usize,
       ab:[(usize,Usize1);n],
       cd:[(Usize1,Usize1);q],
    }
    let num = 2 * 100_000;
    let mut school = vec![BTreeSet::new(); num];
    // どの生徒がどの保育園にいるか保持
    let mut student = vec![];
    for i in 0..n {
        school[ab[i].1].insert((ab[i].0, i));
        student.push(ab[i].1);
    }
    let mut seg = SegmentTree::new(num, 1 << 60, |a, b| a.min(b));
    for i in 0..num {
        if let Some(&(rate, _)) = school[i].iter().last() {
            seg.update(i, rate);
        }
    }
    for (c, d) in cd {
        let a = ab[c].0;
        let b = student[c];
        school[b].remove(&(a, c));
        student[c] = d;
        school[d].insert((a, c));
        let b_val = school[b].iter().last().unwrap_or(&(1 << 60, 0)).0;
        let d_val = school[d].iter().last().unwrap().0;
        seg.update(b, b_val);
        seg.update(d, d_val);
        println!("{}", seg.query(0..num));
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
