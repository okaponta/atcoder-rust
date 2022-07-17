use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,
        ab:[(Usize1,Usize1);n],
    }
    let mut ans = vec![0; m + 1];
    let mut least = vec![0; m];
    let mut maxa = 0;
    let mut minb = m;
    for (a, b) in ab {
        maxa = maxa.max(a);
        minb = minb.min(b);
        least[a] = least[a].max(b);
    }
    let mut r = maxa;
    for l in 0..=minb {
        ans[r - l] += 1;
        ans[m - l] -= 1;
        r = r.max(least[l]);
    }
    for i in 1..=m {
        ans[i] += ans[i - 1];
    }
    ans.remove(m);
    println!("{}", ans.iter().join(" "));
}
