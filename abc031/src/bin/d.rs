use std::{collections::HashMap, process::exit};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        k:usize,
        n:usize,
        vw:[(Chars, Chars);n],
    }
    let mut len = vec![1; k];
    len[0] = 0;
    while next(&mut len) {
        if vw.iter().all(|(v, w)| {
            v.iter()
                .map(|&c| len[(c as u8 - b'1') as usize])
                .sum::<usize>()
                == w.len()
        }) {
            solve(k, &vw, &len);
        }
    }
}

fn solve(k: usize, vw: &Vec<(Vec<char>, Vec<char>)>, len: &Vec<usize>) {
    let mut map: HashMap<char, Vec<char>> = HashMap::new();
    for (v, w) in vw {
        let mut s = vec![];
        let mut i = 0;
        for &c in v {
            if let Some(ww) = map.get(&c) {
                for j in 0..ww.len() {
                    s.push(w[i + j]);
                }
                i += ww.len();
            } else {
                let mut tmp = vec![];
                for j in 0..len[(c as u8 - b'1') as usize] {
                    tmp.push(w[i + j]);
                    s.push(w[i + j]);
                }
                i += tmp.len();
                map.insert(c, tmp);
            }
        }
        if w != &s {
            return;
        }
    }
    for i in 1..=k {
        if let Some(w) = map.get(&std::char::from_digit(i as u32, 10).unwrap()) {
            println!("{}", w.iter().join(""));
        } else {
            println!("a");
        }
    }
    exit(0);
}

fn next(prev: &mut Vec<usize>) -> bool {
    prev[0] += 1;
    for i in 0..prev.len() {
        if prev[i] == 4 {
            if i == prev.len() - 1 {
                return false;
            }
            prev[i] = 1;
            prev[i + 1] += 1;
        }
    }
    true
}
