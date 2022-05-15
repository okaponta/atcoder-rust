use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[i64;n],
    }
    let mut fw = FenwickTree::new(n);
    for i in 0..n {
        fw.add(i + 1, a[i]);
    }
    for _ in 0..q {
        input! {
            query: u8,
        }
        if query == 0 {
            input! {
                p:usize,
                x:i64,
            }
            fw.add(p + 1, x);
        } else {
            input! {
                l:usize,
                r:usize,
            }
            println!("{}", fw.sum(r) - fw.sum(l));
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
