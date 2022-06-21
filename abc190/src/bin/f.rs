use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    // 転倒数を求める
    let mut fw = FenwickTree::new(n);
    let mut inv = 0;
    for i in 0..n {
        inv += i as i64 - fw.sum(a[i] + 1);
        fw.add(a[i] + 1, 1);
    }
    println!("{}", inv);
    for i in 0..n - 1 {
        inv += n as i64 - 2 * a[i] as i64 - 1;
        println!("{}", inv);
    }
}

struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

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
