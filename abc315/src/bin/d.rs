use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[Chars;h],
    }
    let mut col = vec![];
    let mut row = vec![];
    for _ in 0..h {
        col.push(SegmentTree::new(w, (1 << 27) - 1, |a, b| a & b));
    }
    for _ in 0..w {
        row.push(SegmentTree::new(h, (1 << 27) - 1, |a, b| a & b));
    }
    for i in 0..h {
        for j in 0..w {
            let c = (c[i][j] as u8 - b'a' + 1) as usize;
            col[i].update(j, 1usize << c);
            row[j].update(i, 1usize << c);
        }
    }
    let mut remain_col = (0..h).into_iter().collect::<Vec<_>>();
    let mut remain_row = (0..w).into_iter().collect::<Vec<_>>();
    loop {
        let mut done_col = vec![];
        let mut done_row = vec![];
        for &i in &remain_col {
            if remain_row.len() == 1 {
                break;
            }
            if col[i].query(0..w) != 0 {
                done_col.push(i);
            }
        }
        for &i in &remain_row {
            if remain_col.len() == 1 {
                break;
            }
            if row[i].query(0..h) != 0 {
                done_row.push(i);
            }
        }
        if done_col.len() + done_row.len() == 0 {
            break;
        }
        for i in done_col {
            for j in 0..w {
                row[j].update(i, (1 << 27) - 1);
            }
            remain_col.remove(remain_col.binary_search(&i).unwrap());
        }
        for i in done_row {
            for j in 0..h {
                col[j].update(i, (1 << 27) - 1);
            }
            remain_row.remove(remain_row.binary_search(&i).unwrap());
        }
    }
    let ans = remain_col.len() * remain_row.len();
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
