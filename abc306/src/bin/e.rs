use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        q:usize,
        xy:[(Usize1,usize);q],
    }
    let mut psum = PrioritySum::new(&vec![0; n], k);
    for (x, y) in xy {
        psum.update(x, y);
        println!("{}", psum.sum);
    }
}

pub struct PrioritySum {
    a: Vec<usize>,
    big: std::collections::BTreeSet<(usize, usize)>,
    small: std::collections::BTreeSet<(usize, usize)>,
    is_big: Vec<bool>,
    sum: usize,
}

impl PrioritySum {
    fn new(init: &Vec<usize>, k: usize) -> PrioritySum {
        let n = init.len();

        let mut v = init
            .iter()
            .enumerate()
            .map(|(i, e)| (*e, i))
            .collect::<Vec<_>>();
        v.sort();
        v.reverse();

        let a = vec![0; n];
        let mut is_ans = vec![false; n];
        let mut seta = BTreeSet::new();
        let mut setb = BTreeSet::new();
        let mut ans = 0;

        for i in 0..n {
            if i < k {
                seta.insert(v[i]);
                is_ans[v[i].1] = true;
                ans += v[i].0;
            } else {
                setb.insert(v[i]);
            }
        }
        PrioritySum {
            a: a,
            big: seta,
            small: setb,
            is_big: is_ans,
            sum: ans,
        }
    }

    fn update(&mut self, index: usize, value: usize) {
        let before = self.a[index];
        if self.is_big[index] {
            self.big.remove(&(before, index));
            self.big.insert((value, index));
            self.sum -= before;
            self.sum += value;
        } else {
            self.small.remove(&(before, index));
            self.small.insert((value, index));
        }
        self.a[index] = value;
        if self.small.len() != 0 {
            let mina = *self.big.iter().next().unwrap();
            let maxb = *self.small.iter().last().unwrap();
            if mina < maxb {
                self.big.remove(&mina);
                self.small.remove(&maxb);
                self.sum -= mina.0;
                self.big.insert(maxb);
                self.sum += maxb.0;
                self.small.insert(mina);
                self.is_big[mina.1] = false;
                self.is_big[maxb.1] = true;
            }
        }
    }
}
