use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n:Chars,
        k:usize,
    }
    for _ in 0..k {
        n = f(n);
    }
    if n.len() == 0 {
        println!("0");
        return;
    }
    println!("{}", n.into_iter().join(""));
}

fn f(n: Vec<char>) -> Vec<char> {
    let mut m = 0;
    let mut r = 1;
    for i in (0..n.len()).rev() {
        m += n[i].to_digit(10).unwrap() as usize * r;
        r *= 8;
    }
    let mut res = vec![];
    while 0 < m {
        res.push(g(m % 9));
        m /= 9;
    }
    res.reverse();
    res
}

fn g(n: usize) -> char {
    let c = if n % 9 == 8 { 5 } else { n % 9 };
    std::char::from_digit(c as u32, 10).unwrap()
}
