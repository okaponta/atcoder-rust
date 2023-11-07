use proconio::input;

fn main() {
    input! {
        n:usize,
        c:i64,
        xv:[(i64,i64);n],
    }
    let rev_xv = xv
        .iter()
        .rev()
        .map(|(x, v)| (c - x, *v))
        .collect::<Vec<_>>();
    let mut sxv = vec![xv[0]];
    let mut rev_sxv = vec![rev_xv[0]];
    for i in 1..n {
        sxv.push((xv[i].0, xv[i].1 + sxv[i - 1].1));
        rev_sxv.push((rev_xv[i].0, rev_xv[i].1 + rev_sxv[i - 1].1));
    }
    let mut seg = SegmentTree::new(n, 0, |a, b| a.max(b));
    let mut rev_seg = SegmentTree::new(n, 0, |a, b| a.max(b));
    for i in 0..n {
        seg.update_tmp(i, sxv[i].1 - sxv[i].0);
        rev_seg.update_tmp(i, rev_sxv[i].1 - rev_sxv[i].0);
    }
    seg.update_all();
    rev_seg.update_all();
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(sxv[i].1 - sxv[i].0);
        ans = ans.max(rev_sxv[i].1 - rev_sxv[i].0);
        ans = ans.max(sxv[i].1 - 2 * sxv[i].0 + rev_seg.query(0..n - 1 - i));
        ans = ans.max(rev_sxv[i].1 - 2 * rev_sxv[i].0 + seg.query(0..n - 1 - i));
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
