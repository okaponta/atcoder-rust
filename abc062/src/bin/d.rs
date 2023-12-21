use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;3*n],
    }
    let mut first = vec![];
    let mut second = vec![];
    for i in 0..3 * n {
        if i < n {
            first.push(a[i]);
        } else {
            second.push(a[i]);
        }
    }
    first.resize(2 * n, 0);
    let mut sa = PrioritySum::new(&first, n);
    let mut sb = PrioritySumRev::new(&second, n);
    let mut ans = sa.sum as i64 - sb.sum as i64;
    for i in 0..n {
        sa.update(n + i, a[n + i]);
        sb.update(i, 1 << 60);
        ans = ans.max(sa.sum as i64 - sb.sum as i64);
    }
    println!("{}", ans);
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

        let mut a = vec![0; n];
        let mut is_ans = vec![false; n];
        let mut seta = std::collections::BTreeSet::new();
        let mut setb = std::collections::BTreeSet::new();
        let mut ans = 0;

        for i in 0..n {
            a[i] = init[i];
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

    pub fn update(&mut self, index: usize, value: usize) {
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

// 小さいのk個バージョン
pub struct PrioritySumRev {
    a: Vec<usize>,
    small: std::collections::BTreeSet<(usize, usize)>,
    big: std::collections::BTreeSet<(usize, usize)>,
    is_small: Vec<bool>,
    sum: usize,
}

impl PrioritySumRev {
    fn new(init: &Vec<usize>, k: usize) -> PrioritySumRev {
        let n = init.len();

        let mut v = init
            .iter()
            .enumerate()
            .map(|(i, e)| (*e, i))
            .collect::<Vec<_>>();
        v.sort();

        let mut a = vec![0; n];
        let mut is_ans = vec![false; n];
        let mut seta = std::collections::BTreeSet::new();
        let mut setb = std::collections::BTreeSet::new();
        let mut ans = 0;

        for i in 0..n {
            a[i] = init[i];
            if i < k {
                seta.insert(v[i]);
                is_ans[v[i].1] = true;
                ans += v[i].0;
            } else {
                setb.insert(v[i]);
            }
        }
        PrioritySumRev {
            a: a,
            small: seta,
            big: setb,
            is_small: is_ans,
            sum: ans,
        }
    }

    pub fn update(&mut self, index: usize, value: usize) {
        let before = self.a[index];
        if self.is_small[index] {
            self.small.remove(&(before, index));
            self.small.insert((value, index));
            self.sum -= before;
            self.sum += value;
        } else {
            self.big.remove(&(before, index));
            self.big.insert((value, index));
        }
        self.a[index] = value;
        if self.big.len() != 0 {
            let maxa = *self.small.iter().last().unwrap();
            let minb = *self.big.iter().next().unwrap();
            if minb < maxa {
                self.small.remove(&maxa);
                self.big.remove(&minb);
                self.sum -= maxa.0;
                self.small.insert(minb);
                self.sum += minb.0;
                self.big.insert(maxa);
                self.is_small[maxa.1] = false;
                self.is_small[minb.1] = true;
            }
        }
    }
}
