use std::fmt;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        cv: [(Usize1, i64); n],
    }
    let mut dp = vec![Top2::new(); k + 1];
    dp[0].update(0, n);
    for (c, v) in cv {
        let mut next = vec![Top2::new(); k + 1];
        for r in 0..=k {
            for (pv, pc) in dp[r].0 {
                if r != k {
                    next[r + 1].update(pv, pc);
                }
                if pc != c {
                    next[r].update(pv + v, c);
                }
            }
        }
        dp = next;
    }
    println!("{}", dp[k].0[0].0.max(-1));
}

const INF: i64 = std::i64::MAX / 2;
const BAN: usize = 200000 + 2;

#[derive(Clone, Copy)]
struct Top2([(i64, usize); 2]);

impl Top2 {
    fn new() -> Self {
        Self([(-INF, BAN), (-INF, BAN + 1)])
    }
    fn update(&mut self, v: i64, c: usize) {
        let p = &mut self.0;
        if let Some(x) = p.iter().position(|p| p.1 == c) {
            p[x].0 = p[x].0.max(v);
            if p[0].0 < p[1].0 {
                p.swap(0, 1);
            }
        } else {
            for i in 0..2 {
                if v >= p[i].0 {
                    p[i..].rotate_right(1);
                    p[i] = (v, c);
                    break;
                }
            }
        }
    }
}

impl fmt::Debug for Top2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (c, v) in self.0 {
            write!(f, "({}, {})", c, v)?;
        }
        Ok(())
    }
}
