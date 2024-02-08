use itertools::Itertools;
use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i64;n],
        q:usize,
        abc:[(usize,usize,i64);q],
    }
    let mut seg = RangeSegTree::new(n);
    seg.init(&a, 0, 0, n);
    seg.ruiseki();
    let mut bb = 0;
    for (a, b, c) in abc {
        let (a, b, c) = (a ^ bb as usize, b ^ bb as usize, c ^ bb);
        let tmp = seg.query(a - 1, b, c, 0, 0, n);
        println!("{}", tmp);
        bb = tmp;
    }
}

// 領域セグ木
// 各ノードがソート済の配列をもっている
// この実装だと指定の範囲のとある数字以下の数を数えることができる
struct RangeSegTree {
    data: Vec<Vec<i64>>,
    data2: Vec<Vec<i64>>,
}

impl RangeSegTree {
    fn new(size: usize) -> RangeSegTree {
        let m = size.next_power_of_two();
        let data = vec![vec![]; m * 2];
        let data2 = vec![vec![]; m * 2];
        RangeSegTree { data, data2 }
    }

    fn init(&mut self, a: &Vec<i64>, k: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[k].push(a[l]);
        } else {
            let lch = k * 2 + 1;
            let rch = k * 2 + 2;
            let med = (l + r) / 2;
            self.init(a, lch, l, med);
            self.init(a, rch, med, r);
            self.data[k] = self.data[lch]
                .iter()
                .merge(self.data[rch].iter())
                .map(|i| *i)
                .collect::<Vec<i64>>();
        }
    }

    fn ruiseki(&mut self) {
        for i in 0..self.data.len() {
            let mut sum = 0;
            self.data2[i].push(sum);
            for j in 0..self.data[i].len() {
                sum += self.data[i][j];
                self.data2[i].push(sum);
            }
        }
    }

    fn query(&self, i: usize, j: usize, x: i64, k: usize, l: usize, r: usize) -> i64 {
        if j <= l || r <= i {
            0
        } else if i <= l && r <= j {
            self.data2[k][self.data[k].upper_bound(&x)]
        } else {
            let med = (l + r) / 2;
            let lc = self.query(i, j, x, k * 2 + 1, l, med);
            let rc = self.query(i, j, x, k * 2 + 2, med, r);
            lc + rc
        }
    }
}
