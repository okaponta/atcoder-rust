#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut s:Chars,
        xc:[(Usize1,char);q],
    }
    let mut ans = (0..n - 2).into_iter().filter(|&i| f(&s, i)).count();
    s.insert(0, 'Z');
    s.insert(0, 'Z');
    s.push('Z');
    s.push('Z');
    for (x, c) in xc {
        if f(&s, x + 2) || f(&s, x + 1) || f(&s, x) {
            ans -= 1;
        }
        s[x + 2] = c;
        if f(&s, x + 2) || f(&s, x + 1) || f(&s, x) {
            ans += 1;
        }
        println!("{}", ans);
    }
}

fn f(s: &Vec<char>, i: usize) -> bool {
    s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C'
}
