use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n:usize,
        q:usize,
        mut s:Chars,
        qlr:[(usize,Usize1,Usize1);q],
    }
    let mut seg = SegmentTree::new(n, (0, 0), |a, b| (a.0 + b.0, a.1.min(b.1 + a.0)));
    for i in 0..n {
        if s[i] == '(' {
            seg.update(i, (1, 0));
        } else {
            seg.update(i, (-1, -1));
        }
    }
    for (q, l, r) in qlr {
        if q == 1 {
            s.swap(l, r);
            for &i in [l, r].iter() {
                if s[i] == '(' {
                    seg.update(i, (1, 0));
                } else {
                    seg.update(i, (-1, -1));
                }
            }
        } else {
            let (sum, min) = seg.query(l..r + 1);
            println!("{}", if (sum, min) == (0, 0) { "Yes" } else { "No" });
        }
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
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    // 半開区完なので注意
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
