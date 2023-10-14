use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        t:Chars,
        s:[Chars;n],
    }
    let tn = t.len();
    let mut ans = vec![];
    for i in 0..n {
        let sn = s[i].len();
        if sn == tn {
            let mut count = 0;
            for j in 0..sn {
                if s[i][j] != t[j] {
                    count += 1;
                }
            }
            if count < 2 {
                ans.push(i + 1);
            }
        } else if sn + 1 == tn {
            let mut count = 0;
            let mut k = 0;
            for j in 0..tn {
                if sn == k {
                    break;
                }
                if s[i][k] == t[j] {
                    count += 1;
                    k += 1;
                }
            }
            if count == sn {
                ans.push(i + 1);
            }
        } else if sn - 1 == tn {
            let mut count = 0;
            let mut k = 0;
            for j in 0..sn {
                if tn == k {
                    break;
                }
                if s[i][j] == t[k] {
                    count += 1;
                    k += 1;
                }
            }
            if count == tn {
                ans.push(i + 1);
            }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
