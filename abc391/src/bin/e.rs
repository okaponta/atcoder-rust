#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut node = vec![];
    for i in 0..s.len() / 3 {
        node.push(f(s[i * 3], s[i * 3 + 1], s[i * 3 + 2]));
    }
    println!("{}", dfs(1, n, &mut node).1);
}

fn dfs(i: usize, n: usize, node: &mut Vec<(char, usize)>) -> (char, usize) {
    if i == n {
        return node.pop().unwrap();
    }
    let a = dfs(i + 1, n, node);
    let b = dfs(i + 1, n, node);
    let c = dfs(i + 1, n, node);
    let mut zero = vec![];
    let mut one = vec![];
    if a.0 == '0' {
        zero.push(a.1);
    } else {
        one.push(a.1);
    }
    if b.0 == '0' {
        zero.push(b.1);
    } else {
        one.push(b.1);
    }
    if c.0 == '0' {
        zero.push(c.1);
    } else {
        one.push(c.1);
    }
    if zero.len() < one.len() {
        one.sort();
        let mut res = 0;
        for i in 0..one.len() - 1 {
            res += one[i];
        }
        return ('1', res);
    }
    zero.sort();
    let mut res = 0;
    for i in 0..zero.len() - 1 {
        res += zero[i];
    }
    return ('0', res);
}

fn f(a: char, b: char, c: char) -> (char, usize) {
    let mut zero = 0;
    let mut one = 0;
    if a == '0' {
        zero += 1;
    } else {
        one += 1;
    }
    if b == '0' {
        zero += 1;
    } else {
        one += 1;
    }
    if c == '0' {
        zero += 1;
    } else {
        one += 1;
    }
    if zero < one {
        return ('1', one - 1);
    }
    ('0', zero - 1)
}
