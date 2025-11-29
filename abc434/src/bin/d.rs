#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        udlr:[(Usize1,usize,Usize1,usize);n],
    }
    let mut count = TwoDImos::new(2000, 2000);
    let mut sum = TwoDImos::new(2000, 2000);
    for i in 0..n {
        let min = (udlr[i].0, udlr[i].2);
        let max = (udlr[i].1, udlr[i].3);
        count.add(min, max, 1);
        sum.add(min, max, i as i64);
    }
    count.execute();
    sum.execute();
    let mut base = 2000 * 2000;
    let mut ans = vec![0; n];
    for i in 0..2000 {
        for j in 0..2000 {
            if count.s[i][j] == 1 {
                ans[sum.s[i][j] as usize] += 1;
            }
            if 0 < count.s[i][j] {
                base -= 1;
            }
        }
    }
    for ans in ans {
        println!("{}", base + ans);
    }
}

pub struct TwoDImos {
    h: usize,
    w: usize,
    s: Vec<Vec<i64>>,
}

impl TwoDImos {
    pub fn new(h: usize, w: usize) -> Self {
        let s = vec![vec![0; w + 1]; h + 1];
        TwoDImos { h, w, s }
    }

    pub fn add(&mut self, min: (usize, usize), max: (usize, usize), num: i64) {
        self.s[min.0][min.1] += num;
        self.s[min.0][max.1] -= num;
        self.s[max.0][min.1] -= num;
        self.s[max.0][max.1] += num;
    }

    pub fn execute(&mut self) {
        for i in 0..=self.h {
            for j in 1..=self.w {
                self.s[i][j] += self.s[i][j - 1];
            }
        }
        for j in 0..=self.w {
            for i in 1..=self.h {
                self.s[i][j] += self.s[i - 1][j];
            }
        }
    }
}
