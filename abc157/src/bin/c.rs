use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        sc:[(Usize1,i32);m],
    }
    let mut ans = vec![-1; n];
    for (s, c) in sc {
        if ans[s] == -1 || ans[s] == c {
            ans[s] = c;
        } else {
            println!("-1");
            return;
        }
    }
    if ans[0] == 0 {
        if n == 1 {
            println!("0");
            return;
        }
        println!("-1");
        return;
    }
    if ans[0] == -1 {
        if n == 1 {
            println!("0");
            return;
        }
        ans[0] = 1;
    }
    for i in 1..n {
        if ans[i] == -1 {
            ans[i] = 0;
        }
    }
    println!("{}", ans.iter().join(""));
}
