use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s:Chars,
        q:usize,
        tk:[(usize,Usize1);q],
    }
    let si = s.iter().map(|c| (*c as u8 - b'A') as i64).collect_vec();
    for (ti, ki) in tk {
        let target = func(ti, ki, 0);
        let ans = if target.0 == 0 {
            (si[target.1] + target.2) % 3
        } else {
            (si[0] + target.0 as i64 + target.2) % 3
        };
        println!("{}", (b'A' + ans as u8) as char);
    }
}

fn func(t: usize, k: usize, count: i64) -> (usize, usize, i64) {
    if t == 0 || k == 0 {
        return (t, k, count);
    }
    func(t - 1, k / 2, count + 1 + k as i64 % 2)
}
