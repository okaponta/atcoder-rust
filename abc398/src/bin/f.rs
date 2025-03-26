#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut t = vec![];
    for i in (0..n).rev() {
        t.push(s[i]);
    }
    t.push('#');
    for i in 0..n {
        t.push(s[i]);
    }
    let m = 2 * n + 1;
    let z = z(&t);
    let mut from = 0;
    for i in n + 1..m {
        if i + z[i] == m {
            from = z[i];
            break;
        }
    }
    let mut ans = s.clone();
    for i in (0..n - from).rev() {
        ans.push(s[i]);
    }
    println!("{}", ans.iter().join(""));
}

fn z(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n + 1];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
