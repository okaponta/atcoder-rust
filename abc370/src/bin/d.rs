use std::collections::BTreeSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
        rc:[(Usize1,Usize1);q],
    }
    let mut row = vec![(0..w).into_iter().collect::<BTreeSet<_>>(); h];
    let mut col = vec![(0..h).into_iter().collect::<BTreeSet<_>>(); w];
    for (r, c) in rc {
        if row[r].contains(&c) {
            row[r].remove(&c);
            col[c].remove(&r);
        } else {
            if let Some(&c1) = row[r].range(c..).next() {
                row[r].remove(&c1);
                col[c1].remove(&r);
            }
            if let Some(&c2) = row[r].range(..c).last() {
                row[r].remove(&c2);
                col[c2].remove(&r);
            }
            if let Some(&r1) = col[c].range(r..).next() {
                row[r1].remove(&c);
                col[c].remove(&r1);
            }
            if let Some(&r2) = col[c].range(..r).last() {
                row[r2].remove(&c);
                col[c].remove(&r2);
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        ans += row[i].len();
    }
    println!("{}", ans);
}
