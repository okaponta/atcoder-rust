use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        rect:[(usize,usize,usize,usize);n],
    }
    let mut s = TwoDImos::new(1000, 1000);
    for (lx, ly, rx, ry) in rect {
        s.add((lx, ly), (rx, ry), 1);
    }
    s.execute();
    let mut ans = vec![0; n + 1];
    for i in 0..=1000 {
        for j in 0..=1000 {
            ans[s.s[i][j] as usize] += 1;
        }
    }
    for i in 1..=n {
        println!("{}", ans[i]);
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
