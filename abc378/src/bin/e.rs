#[allow(unused)]
use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    let mut s = 0;
    let mut ss = 0;
    let mut ans = 0;
    let mut seg = SegmentTree::new(m, 0, |a, b| a + b);
    seg.update(0, 1);
    for i in 0..n {
        s = (s + a[i]) % m;
        ans += s * (i + 1) + seg.query((s + 1)..m) * m - ss;
        seg.update(s, seg.get(s) + 1);
        ss += s;
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
