use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    if n == 2 {
        println!("1");
        return;
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for i in divisor(n) {
        set.insert(i);
        let mut tmp = n;
        while tmp % i == 0 {
            tmp /= i;
        }
        if tmp % i == 1 {
            ans += 1;
        }
    }
    for i in divisor(n - 1) {
        if !set.contains(&i) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut upper = vec![n];
    for i in 2..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}
