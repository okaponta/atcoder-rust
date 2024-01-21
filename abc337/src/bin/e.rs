use itertools::Itertools;
use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive! {
        n:usize,
    }
    let m = n.next_power_of_two().trailing_zeros() as usize;
    println!("{}", m);
    let mut res = vec![vec![]; m];
    for i in 0..n {
        for j in 0..m {
            if i >> j & 1 == 1 {
                res[j].push(i + 1);
            }
        }
    }
    for i in 0..m {
        println!("{} {}", res[i].len(), res[i].iter().join(" "));
    }
    input_interactive! {
        mut s: Chars,
    }
    let mut ans = 1;
    let mut d = 1;
    for i in 0..m {
        if s[i] == '1' {
            ans += d;
        }
        d *= 2;
    }
    println!("{}", ans);
}
