use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut seg = SegmentTree::new(n, (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    let order = a
        .iter()
        .enumerate()
        .sorted_by_key(|e| e.1)
        .rev()
        .collect::<Vec<_>>();
    let mut ans = 0;
    for (i, &a) in order {
        let (sum, count) = seg.query(i + 1..n);
        ans += sum - count * a;
        seg.update(i, (a, 1));
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
