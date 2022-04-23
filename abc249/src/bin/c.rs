use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:[Chars;n],
    }
    let mut sets = vec![];
    for ch in s {
        let mut set = HashSet::new();
        for c in ch {
            set.insert(c);
        }
        sets.push(set);
    }
    let mut ans = 0;
    for i in 0..1 << n {
        let mut count = vec![0; 26];
        for j in 0..n {
            if i >> j & 1 == 1 {
                for k in 0..26 {
                    let ch = (b'a' + k as u8) as char;
                    if sets[j].contains(&ch) {
                        count[k] += 1;
                    }
                }
            }
        }
        let tmp = count.iter().filter(|i| i == &&k).count();
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
