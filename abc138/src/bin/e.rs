use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let mut map = vec![vec![]; 26];
    for i in 0..s.len() {
        let alpha = (s[i] as u8 - b'a') as usize;
        map[alpha].push(i);
    }
    let mut quotient = 0;
    let mut rem = 0;
    for i in 0..t.len() {
        let alpha = (t[i] as u8 - b'a') as usize;
        if map[alpha].len() == 0 {
            println!("-1");
            return;
        }
        let index = map[alpha].lower_bound(&rem);
        if map[alpha].len() == index {
            quotient += 1;
            rem = map[alpha][0] + 1;
        } else {
            rem = map[alpha][index] + 1;
        }
    }
    println!("{}", quotient * s.len() + rem);
}
