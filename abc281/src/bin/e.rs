use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        a:[usize;n],
    }
    let mut l = BTreeSet::new();
    let mut r = BTreeSet::new();
    let mut first = (0..m).into_iter().map(|i| (a[i], i)).collect::<Vec<_>>();
    first.sort();
    let mut tmp = 0;
    for i in 0..m {
        if i < k {
            l.insert(first[i]);
            tmp += first[i].0;
        } else {
            r.insert(first[i]);
        }
    }
    let mut ans = vec![tmp];
    for i in m..n {
        if l.remove(&(a[i - m], i - m)) {
            tmp -= a[i - m];
        }
        r.remove(&(a[i - m], i - m));
        let (lmax, _) = l.iter().last().unwrap_or(&(1 << 60, 0));
        if lmax < &a[i] {
            r.insert((a[i], i));
        } else {
            l.insert((a[i], i));
            tmp += a[i];
        }
        if l.len() < k {
            // 1つ補充
            let &(rmin, rmini) = r.iter().next().unwrap();
            r.remove(&(rmin, rmini));
            l.insert((rmin, rmini));
            tmp += rmin;
        }
        if k < l.len() {
            // 1つ放出
            let &(lmax, lmaxi) = l.iter().last().unwrap();
            l.remove(&(lmax, lmaxi));
            tmp -= lmax;
            r.insert((lmax, lmaxi));
        }
        ans.push(tmp);
    }
    println!("{}", ans.iter().join(" "))
}
