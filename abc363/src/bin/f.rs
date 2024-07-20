use std::{collections::HashSet, process::exit};

use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    dfs(&mut vec![], &mut vec![], n, &mut HashSet::new());
    println!("-1");
}

fn dfs(cur: &mut Vec<usize>, rev: &mut Vec<usize>, n: usize, ng: &mut HashSet<usize>) {
    if is_kai(n) && !contains_zero(n) {
        cur.push(n);
        println!("{}", cur.iter().chain(rev.iter().rev()).join("*"));
        exit(0);
    }
    for i in 2..=n.sqrt() {
        if n % i != 0 || contains_zero(i) {
            continue;
        }
        let revi = rev_num(i);
        if n % (i * revi) != 0 {
            continue;
        }
        cur.push(i);
        rev.push(revi);
        if !ng.contains(&(n / (i * revi))) {
            dfs(cur, rev, n / (i * revi), ng);
            ng.insert(n / (i * revi));
        }
        cur.pop();
        rev.pop();
    }
}

fn rev_num(n: usize) -> usize {
    n.to_string()
        .chars()
        .into_iter()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn is_kai(n: usize) -> bool {
    let s = n.to_string().chars().collect::<Vec<_>>();
    let l = s.len();
    (0..l).all(|i| s[i] == s[l - 1 - i])
}

fn contains_zero(n: usize) -> bool {
    let s = n.to_string().chars().collect::<Vec<_>>();
    let l = s.len();
    (0..l).any(|i| s[i] == '0')
}
