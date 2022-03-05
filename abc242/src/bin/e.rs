use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t:i32,
    }
    for _ in 0..t {
        testcase();
    }
}

fn testcase() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = 0;
    for i in 0..((n + 1) / 2) {
        let num = (s[i as usize] as u8 - b'A') as i64;
        ans = ans * 26 + num;
        ans %= 998244353;
    }
    let ss = s.iter().collect_vec();
    let mut target = ss.clone();
    for i in n / 2..n {
        target[i] = target[n - 1 - i];
    }
    if target <= ss {
        ans += 1;
    }
    println!("{}", ans % 998244353);
}
