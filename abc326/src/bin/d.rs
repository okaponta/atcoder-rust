use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        r:Chars,
        c:Chars,
    }
    if !contains_abc(&r) || !contains_abc(&c) {
        println!("No");
        return;
    }
    let chars = vec!['A', 'B', 'C', '.', '.'];
    let mut rows = vec![];
    for p in (0..n).permutations(n) {
        let mut tmp = vec![];
        for i in 0..n {
            tmp.push(chars[p[i]]);
        }
        rows.push(tmp);
    }
    rows.sort();
    rows.dedup();
    let mut q = VecDeque::new();
    let count = vec![vec![0; 3]; n];
    let ans = vec![];
    q.push_back((ans, count));
    while let Some((ans, count)) = q.pop_front() {
        for i in 0..rows.len() {
            let pa = rows[i].iter().position(|&c| c == 'A').unwrap();
            let pb = rows[i].iter().position(|&c| c == 'B').unwrap();
            let pc = rows[i].iter().position(|&c| c == 'C').unwrap();
            if count[pa][0] < 1 && count[pb][1] < 1 && count[pc][2] < 1 {
                let mut tmp = ans.clone();
                let mut tmp_count = count.clone();
                tmp_count[pa][0] += 1;
                tmp_count[pb][1] += 1;
                tmp_count[pc][2] += 1;
                tmp.push(rows[i].clone());
                if tmp.len() < n {
                    q.push_back((tmp, tmp_count));
                } else {
                    let mut rr = vec![];
                    let mut cc = vec![];
                    for i in 0..n {
                        let mut j = 0;
                        while tmp[i][j] == '.' {
                            j += 1;
                        }
                        rr.push(tmp[i][j]);
                    }
                    for j in 0..n {
                        let mut i = 0;
                        while tmp[i][j] == '.' {
                            i += 1;
                        }
                        cc.push(tmp[i][j]);
                    }
                    if rr == r && cc == c {
                        println!("Yes");
                        for i in 0..n {
                            println!("{}", tmp[i].iter().join(""));
                        }
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}

fn contains_abc(s: &Vec<char>) -> bool {
    s.contains(&'A') && s.contains(&'B') && s.contains(&'C')
}
