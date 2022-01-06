use itertools::Itertools;
use proconio::input;

fn next(prev: i32) -> i32 {
    let big = prev
        .to_string()
        .chars()
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let small = prev
        .to_string()
        .chars()
        .sorted_by(|a, b| Ord::cmp(&a, &b))
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    big - small
}

fn main() {
    input! {
       n:i32,k:i32,
    }
    let mut ans = n;
    for _i in 0..k {
        ans = next(ans);
    }
    println!("{}", ans);
}
