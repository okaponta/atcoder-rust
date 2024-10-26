use std::collections::HashMap;

#[allow(unused)]
use itertools::*;
use petgraph::unionfind::UnionFind;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        mut k:usize,
        p:[Usize1;n],
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.union(i, p[i]);
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(uf.find(i)).or_insert(vec![]).push(i);
    }
    let mut pp = vec![];
    pp.push((0..n).into_iter().collect::<Vec<_>>());
    let mut prev = p.clone();
    for _ in 0..n.next_power_of_two().count_zeros() + 3 {
        let mut next = (0..n).into_iter().collect::<Vec<_>>();
        for i in 0..n {
            next[i] = prev[prev[i]];
        }
        pp.push(prev);
        prev = next;
    }
    let mut answer = (0..n).into_iter().collect::<Vec<_>>();
    for (_, v) in map {
        let mut tmp = 0;
        let two = pow(2, k, v.len());
        tmp += two;
        let mut count = 1;
        let mut ans = (0..n).into_iter().collect::<Vec<_>>();
        while 0 < tmp {
            if tmp & 1 == 1 {
                let mut next = (0..n).into_iter().collect::<Vec<_>>();
                for i in 0..n {
                    next[i] = pp[count][ans[i]];
                }
                ans = next;
            }
            tmp /= 2;
            count += 1;
        }
        for i in v {
            answer[i] = ans[i];
        }
    }
    println!("{}", answer.iter().map(|i| i + 1).join(" "));
}

fn pow(mut x: usize, mut n: usize, modulo: usize) -> usize {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
