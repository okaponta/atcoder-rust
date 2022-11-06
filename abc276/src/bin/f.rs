use proconio::{fastout, input};

const MOD: i64 = 998244353;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut sorted_a = vec![];
    for i in 0..n {
        sorted_a.push((a[i], i));
    }
    sorted_a.sort();
    for i in 0..n {
        sorted_a[i] = (sorted_a[i].1, i + 1);
    }
    sorted_a.sort();
    let mut fw = FenwickTree::new(n);
    let mut count = FenwickTree::new(n);
    let mut numerator = 0;
    for i in 0..n {
        let ai = a[i] as i64;
        let index = sorted_a[i].1;
        numerator += fw.range(index, n) * 2 % MOD;
        numerator += (count.sum(index) * 2 + 1) * ai % MOD;
        fw.add(index, ai);
        count.add(index, 1);
        numerator %= MOD;
        let ans = numerator * modinv(((i + 1) * (i + 1)) as i64, MOD) % MOD;
        println!("{}", ans);
    }
}

fn modinv(mut a: i64, modulo: i64) -> i64 {
    let mut b = modulo;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= modulo;
    if u < 0 {
        u += modulo;
    }
    u
}

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

    // ai+...+ajを計算する
    fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }
}
