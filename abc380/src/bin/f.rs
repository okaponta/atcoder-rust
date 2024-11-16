use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        l:usize,
        mut a:[usize;n],
        mut b:[usize;m],
        mut c:[usize;l],
    }
    let mut d = vec![];
    a.sort();
    b.sort();
    c.sort();
    for i in 0..n {
        d.push(a[i]);
    }
    for i in 0..m {
        d.push(b[i]);
    }
    for i in 0..l {
        d.push(c[i]);
    }
    d.sort();
    let mut three = 1;
    let mut cv = HashMap::new();
    for i in 0..n + m + l {
        cv.insert(d[i], three);
        three *= 3;
    }
    let mut memo = HashMap::new();
    println!(
        "{}",
        if dfs(a, b, c, &mut memo, &cv) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

fn encode(b: &Vec<usize>, c: &Vec<usize>, cv: &HashMap<usize, usize>) -> usize {
    let mut res = 0;
    for b in b {
        res += cv[b];
    }
    for c in c {
        res += cv[c] * 2;
    }
    res
}

fn dfs(
    a: Vec<usize>,
    b: Vec<usize>,
    c: Vec<usize>,
    memo: &mut HashMap<usize, bool>,
    cv: &HashMap<usize, usize>,
) -> bool {
    let key = encode(&b, &c, cv);
    if let Some(&bool) = memo.get(&key) {
        return bool;
    }
    if a.len() == 0 {
        memo.insert(key, false);
        return false;
    }
    let n = a.len();
    for i in 0..n {
        let mut nexta = a.clone();
        let mut nextc = c.clone();
        let bef = nexta.remove(i);
        nextc.push(bef);
        nextc.sort();
        if let Some(j) = nextc.iter().rposition(|&c| c < bef) {
            let cbef = nextc.remove(j);
            nexta.push(cbef);
        }
        nexta.sort();
        let bool = dfs(b.clone(), nexta, nextc, memo, cv);
        if !bool {
            memo.insert(key, true);
            return true;
        }
    }
    memo.insert(key, false);
    false
}
