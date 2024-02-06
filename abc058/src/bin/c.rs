use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut count = vec![100; 26];
    for i in 0..n {
        let mut tmp = vec![0; 26];
        for j in 0..s[i].len() {
            tmp[(s[i][j] as u8 - b'a') as usize] += 1;
        }
        for j in 0..26 {
            count[j] = count[j].min(tmp[j]);
        }
    }
    let mut ans = vec![];
    for i in 0..26 {
        for _ in 0..count[i] {
            ans.push((b'a' + i as u8) as char);
        }
    }
    println!("{}", ans.iter().join(""));
}
