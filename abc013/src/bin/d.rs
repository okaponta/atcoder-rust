use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut d:usize,
        a:[usize;m],
    }
    let mut cv = (0..n).into_iter().collect::<Vec<_>>();
    let mut ans = cv.clone();
    for a in a {
        cv.swap(a - 1, a);
    }
    while 0 < d {
        if d & 1 == 1 {
            let mut tmp = vec![0; n];
            for i in 0..n {
                tmp[cv[i]] = ans[i];
            }
            ans = tmp;
        }
        cv = (0..n).into_iter().map(|i| cv[cv[i]]).collect();
        d /= 2;
    }
    println!("{}", ans.iter().map(|i| i + 1).join("\n"));
}
