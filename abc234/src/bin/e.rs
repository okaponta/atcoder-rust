use std::collections::HashSet;

use proconio::input;
use superslice::Ext;

fn gen_tosasu() -> Vec<i64> {
    let mut set = HashSet::new();
    for first in 1..10 {
        for d in -9..9 {
            let mut s = String::new();
            let mut digit = first;
            for _ in 0..18 {
                s.push(std::char::from_digit(digit as u32, 10).unwrap());
                set.insert(s.parse::<i64>().unwrap());
                digit += d;
                if !(0 <= digit && digit <= 9) {
                    break;
                }
            }
        }
    }
    let mut res = set.into_iter().collect::<Vec<_>>();
    res.sort();
    res
}

fn main() {
    input! {x:i64 }
    let tosasu = gen_tosasu();
    println!("{}", tosasu[tosasu.lower_bound(&x)]);
}
