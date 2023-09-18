use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        m:usize,
        s:[Chars;3],
    }
    let mut ans = 4 * m;
    for i in 0..10 {
        for p in (0..3).permutations(3) {
            let mut flg = vec![false; 3];
            for t in 0..4 * m {
                for &j in &p {
                    if !flg[j] && s[j][t % m] == (b'0' + i) as char {
                        flg[j] = true;
                        break;
                    }
                }
                if flg.iter().all(|b| *b) {
                    ans = ans.min(t);
                    break;
                }
            }
        }
    }
    if ans == 4 * m {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
