#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        mut rc:[(Usize1,Usize1);n],
    }
    rc.push((0, 0));
    rc.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut seg = SegmentTree::new(w, (0, 0), |a, b| if a.0 < b.0 { b } else { a });
    let mut parent = vec![0; n + 1];
    for i in 0..=n {
        let (v, pre) = seg.query(0..rc[i].1 + 1);
        parent[i] = pre;
        seg.update(rc[i].1, (v + 1, i));
    }
    let (ans, mut p) = seg.query(0..w);
    let mut hi = h - 1;
    let mut wi = w - 1;
    let mut path = vec![];
    while p != 0 {
        while wi != rc[p].1 {
            path.push('R');
            wi -= 1;
        }
        while hi != rc[p].0 {
            path.push('D');
            hi -= 1;
        }
        p = parent[p];
    }
    while wi != 0 {
        path.push('R');
        wi -= 1;
    }
    while hi != 0 {
        path.push('D');
        hi -= 1;
    }
    path.reverse();
    println!("{}", ans - 1);
    println!("{}", path.iter().join(""));
}

// segment tree
// seg[0] -> seg[1]+seg[2]
// seg[1] -> seg[3]+seg[4] seg[2] -> seg[5]+seg[6]
// seg[3] -> seg[7]+seg[8] seg[6] -> seg[13]+seg[14]
// 必要な要素数は2^n-1
// 区間上の値を更新する
// 任意の区間上の最小値や合計値(与えるfuncによって全てのbit or値)などを取得する
// let mut seg = SegmentTree::new(9, 0, |a, b| a + b);
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

    // lowerとupperの間でfを満たす最小の値
    // ng, ng, ng, (ok), ok, ok
    pub fn max_right<P>(&self, mut lower: usize, mut upper: usize, f: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if f(self.query(0..med)) {
                upper = med;
            } else {
                lower = med;
            }
        }
        upper
    }

    // lowerとupperの間でfを満たさない最大の値
    pub fn min_left<P>(&self, mut lower: usize, mut upper: usize, f: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if f(self.query(0..med)) {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}

impl<T: Copy + std::fmt::Debug, F> std::fmt::Debug for SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = vec![];
        for i in 0..self.n {
            v.push(self.get(i));
        }
        writeln!(f, "{:?}", v)?;
        Ok(())
    }
}