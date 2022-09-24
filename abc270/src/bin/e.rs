use itertools::{self, Itertools};
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    // 各カゴから減らすりんごの個数
    let mut lower = 0;
    let mut upper = *a.iter().max().unwrap();
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if countup(med, &a) <= k {
            lower = med;
        } else {
            upper = med;
        }
    }
    let mut rem = k - countup(lower, &a);
    let mut ans = vec![0; n];
    for i in 0..n {
        if a[i] < lower {
            ans[i] = 0;
        } else {
            ans[i] = a[i] - lower;
            if 0 < rem && 0 < ans[i] {
                ans[i] -= 1;
                rem -= 1;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}

fn countup(apple: usize, a: &Vec<usize>) -> usize {
    let mut count = 0;
    for ai in a {
        if ai < &apple {
            count += ai;
        } else {
            count += apple;
        }
    }
    count
}
