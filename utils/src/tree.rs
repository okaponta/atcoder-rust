// segment tree。
// 区間上の値を更新する
// 任意の区間上の最小値や合計値(与えるfuncによって全てのbit or値)などを取得する
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

// フェニック木。以下2つができる
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment() {
        let mut seg = SegmentTree::new(9, 0, |a, b| a + b);
        seg.update(1, 1);
        assert_eq!(seg.query(0..1), 0);
        assert_eq!(seg.query(0..2), 1);

        seg.update(5, 2);
        assert_eq!(seg.query(0..5), 1);
        assert_eq!(seg.query(0..6), 3);
        assert_eq!(seg.query(5..6), 2);

        seg.update(9, 1);
        assert_eq!(seg.query(0..9), 3);
        assert_eq!(seg.query(0..10), 4);
        assert_eq!(seg.query(9..10), 1);

        seg.update(2, -3);
        assert_eq!(seg.query(0..2), 1);
        assert_eq!(seg.query(0..3), -2);
        assert_eq!(seg.query(0..10), 1);
    }

    #[test]
    fn test_fenwick() {
        let mut fw = FenwickTree::new(9);
        fw.add(1, 1);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(0), 0);
        assert_eq!(fw.sum(1), 1);
        assert_eq!(fw.sum(2), 1);
        fw.add(5, 2);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(4), 1);
        assert_eq!(fw.sum(5), 3);
        fw.add(9, 1);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(8), 3);
        assert_eq!(fw.sum(9), 4);
        fw.add(2, -3);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(1), 1);
        assert_eq!(fw.sum(2), -2);
        assert_eq!(fw.sum(9), 1);
    }
}
