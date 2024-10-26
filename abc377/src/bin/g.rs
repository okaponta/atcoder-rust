use std::collections::HashMap;

use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut map = HashMap::new();
    let base = 1000001137;
    for s in s {
        let mut ans = s.len();
        let mut tmp = 0;
        let mut b = 1;
        for i in 0..s.len() {
            let next = (s[i] as u128).wrapping_mul(b).wrapping_add(tmp);
            b = b.wrapping_mul(base);
            if let Some(&(common, len)) = map.get(&next) {
                ans = ans.min(len + s.len() - 2 * common);
                if s.len() < len {
                    map.insert(next, (common, s.len()));
                }
            } else {
                map.insert(next, (i + 1, s.len()));
            }
            tmp = next;
        }
        println!("{}", ans);
    }
}
