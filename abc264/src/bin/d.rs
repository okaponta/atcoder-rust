use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut v = vec![0; 7];
    for i in 0..7 {
        if s[i] == 'a' {
            v[i] = 1;
        }
        if s[i] == 't' {
            v[i] = 2;
        }
        if s[i] == 'c' {
            v[i] = 3;
        }
        if s[i] == 'o' {
            v[i] = 4;
        }
        if s[i] == 'd' {
            v[i] = 5;
        }
        if s[i] == 'e' {
            v[i] = 6;
        }
        if s[i] == 'r' {
            v[i] = 7;
        }
    }
    println!("{}", inversion_num(v));
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
}

// 転倒数を求める。
// とりうる最大値の木作るので、制約が大きい場合は座標圧縮などしてから呼び出すこと
fn inversion_num(a: Vec<usize>) -> i64 {
    let max = *a.iter().max().unwrap();
    let mut inv = 0;
    let mut fw = FenwickTree::new(max);
    // 注意：1-indexedで行うこと！
    for i in 0..a.len() {
        inv += i as i64 - fw.sum(a[i]);
        fw.add(a[i], 1);
    }
    inv
}
