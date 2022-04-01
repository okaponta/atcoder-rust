use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:[Chars;3],
    }
    let mut map = HashMap::new();
    let mut count = 0;
    for i in 0..3 {
        for c in &s[i] {
            if !map.contains_key(&c) {
                map.insert(c, count);
                count += 1;
            }
        }
    }
    let len = map.keys().len();
    if len > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let mut si = vec![vec![]; 3];
    for i in 0..3 {
        for c in &s[i] {
            si[i].push(map.get(c).unwrap());
        }
    }
    for perm in (0..10).into_iter().permutations(len) {
        if perm[*si[0][0]] == 0 || perm[*si[1][0]] == 0 || perm[*si[2][0]] == 0 {
            continue;
        }
        let n1 = convert(&si[0], &perm);
        let n2 = convert(&si[1], &perm);
        let n3 = convert(&si[2], &perm);
        if n1 + n2 == n3 {
            println!("{}", n1);
            println!("{}", n2);
            println!("{}", n3);
            return;
        }
    }
    println!("UNSOLVABLE");
}

fn convert(s: &Vec<&usize>, map: &Vec<i64>) -> i64 {
    let mut res = 0;
    for &&digit in s.iter() {
        res *= 10;
        res += map[digit];
    }
    res
}
