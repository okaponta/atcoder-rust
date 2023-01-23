use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut s:Chars,
        q:usize,
    }
    let mut t = [0; 26];
    let mut seg = SegmentTree::new(n, ([0; 26], true), |a, b| merge(a, b));
    for i in 0..n {
        let c = (s[i] as u8 - b'a') as usize;
        t[c] += 1;
        let mut e = ([0; 26], true);
        e.0[c] += 1;
        seg.update_tmp(i, e);
    }
    seg.update_all();
    for _ in 0..q {
        input! {a: u8}
        if a == 1 {
            input! {x:Usize1, c:char}
            let bef = (s[x] as u8 - b'a') as usize;
            s[x] = c;
            let after = (c as u8 - b'a') as usize;
            t[bef] -= 1;
            t[after] += 1;
            let mut e = ([0; 26], true);
            e.0[after] += 1;
            seg.update(x, e);
        } else {
            input! {l:Usize1,r:usize}
            let ans = is_ok(seg.query(l..r), t);
            println!("{}", if ans { "Yes" } else { "No" });
        }
    }
}

fn is_ok(a: ([usize; 26], bool), t: [usize; 26]) -> bool {
    if !a.1 {
        return false;
    }
    let a_edge = calc_edge(a.0);
    let t_edge = calc_edge(t);
    if a_edge.0 < t_edge.0 || t_edge.1 < a_edge.1 {
        return false;
    }
    for i in a_edge.0..=a_edge.1 {
        if i == a_edge.0 || i == a_edge.1 {
            if t[i] < a.0[i] {
                return false;
            }
        } else {
            if t[i] != a.0[i] {
                return false;
            }
        }
    }
    true
}

fn merge(a: ([usize; 26], bool), b: ([usize; 26], bool)) -> ([usize; 26], bool) {
    let mut c = [0; 26];
    for i in 0..26 {
        c[i] = a.0[i] + b.0[i];
    }
    (c, is_ordered(a, b))
}

fn is_ordered(a: ([usize; 26], bool), b: ([usize; 26], bool)) -> bool {
    if !a.1 || !b.1 {
        return false;
    }
    let a_edge = calc_edge(a.0);
    let b_edge = calc_edge(b.0);
    a_edge.1 <= b_edge.0
}

fn calc_edge(a: [usize; 26]) -> (usize, usize) {
    let start = (0..26).into_iter().find(|&i| a[i] != 0).unwrap_or(26);
    let end = (0..26).into_iter().rev().find(|&i| a[i] != 0).unwrap_or(0);
    (start, end)
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
