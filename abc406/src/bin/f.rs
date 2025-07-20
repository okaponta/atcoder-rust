#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
        q:usize,
    }
    let mut sum = n;
    let mut g = vec![vec![]; n];
    let mut xmap = vec![0; n];
    let mut ymap = vec![vec![]; n];
    for i in 0..n - 1 {
        let (u, v) = uv[i];
        g[u].push((v, i));
        g[v].push((u, i));
    }
    dfs(0, 0, &g, 0, &mut ymap, &mut xmap);
    let mut seg = SegmentTree::new(2 * n, 0, |a, b| a + b);
    for &i in &xmap {
        seg.update(i, 1);
    }
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {x: Usize1, w:usize}
            seg.update(xmap[x], seg.get(xmap[x]) + w);
            sum += w;
        } else {
            input! {y:Usize1}
            let a = seg.query(ymap[y][0]..ymap[y][1]);
            println!("{}", abs(a, sum - a));
        }
    }
}

fn abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn dfs(
    cur: usize,
    prev: usize,
    g: &Vec<Vec<(usize, usize)>>,
    count: usize,
    ymap: &mut Vec<Vec<usize>>,
    xmap: &mut Vec<usize>,
) -> usize {
    let mut res = count;
    xmap[cur] = res;
    for &(next, idx) in &g[cur] {
        if next == prev {
            continue;
        }
        ymap[idx].push(res + 1);
        res = dfs(next, cur, g, res + 1, ymap, xmap);
        ymap[idx].push(res);
    }
    res + 1
}

// segment tree
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
