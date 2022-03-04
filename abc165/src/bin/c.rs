use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,m:usize,q:usize,
       abcd:[(Usize1,Usize1,usize,usize);q],
    }
    let mut ans = 0;
    for a in (0..m).combinations_with_replacement(n) {
        let mut point = 0;
        for &(ai, bi, ci, di) in &abcd {
            if a[bi] - a[ai] == ci {
                point += di;
            }
        }
        ans = ans.max(point);
    }
    print!("{}", ans);
}
