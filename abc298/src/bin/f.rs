use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        rcx:[(Usize1,Usize1,usize);n],
    }
    let mut map = HashMap::new();
    let mut is = vec![];
    let mut js = vec![];
    for (r, c, _) in &rcx {
        is.push(*r);
        js.push(*c);
    }
    let imap = compress(&is);
    let jmap = compress(&js);
    let mut isum = vec![];
    let mut jsum = vec![];
    for i in 0..imap.len() {
        isum.push((0, i));
    }
    for j in 0..jmap.len() {
        jsum.push((0, j));
    }
    for (r, c, x) in rcx {
        (*map.entry(imap[&r]).or_insert(HashMap::new())).insert(jmap[&c], x);
        isum[imap[&r]].0 += x;
        jsum[jmap[&c]].0 += x;
    }
    isum.sort_by(|a, b| b.0.cmp(&a.0));
    jsum.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = 0;
    for (sum, index) in isum {
        let m = map.get(&index).unwrap();
        for (jsum, jindex) in &jsum {
            if sum + jsum <= ans {
                break;
            }
            ans = ans.max(sum + *jsum - m.get(jindex).unwrap_or(&0))
        }
    }
    println!("{}", ans);
}

fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}
