use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        mut b:[usize;n],
    }
    let mut ab = vec![];
    for i in 0..n {
        ab.push((a[i], b[i]));
    }
    ab.sort();
    a.sort();
    b.sort();
    for i in 0..n {
        if a[i] != b[i] {
            println!("No");
            return;
        }
    }
    a.dedup();
    if a.len() != n {
        println!("Yes");
        return;
    }
    let b = ab.into_iter().map(|(_, b)| b).collect::<Vec<_>>();
    println!(
        "{}",
        if inversion_num(b) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
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

pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    pub fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    pub fn sum(&self, i: usize) -> i64 {
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
    pub fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }

    // 和がs以下の位置を返却
    pub fn lower(&self, s: i64) -> usize {
        let mut lower = 0;
        let mut upper = self.len;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if self.sum(med) <= s {
                lower = med;
            } else {
                upper = med;
            }
        }
        lower
    }
}
