#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _k:usize,
        mut s:Chars,
        mut t:Chars,
    }
    s.push('#');
    t.push('#');
    for i in 0..t.len() {
        if s[i] == t[i] {
            continue;
        }
        if s.len() < t.len() {
            s.insert(i, t[i]);
        } else if s.len() > t.len() {
            s.remove(i);
        } else {
            s[i] = t[i];
        }
        break;
    }
    println!("{}", if s == t { "Yes" } else { "No" });
}
