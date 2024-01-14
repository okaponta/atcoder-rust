use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        mut n:Usize1,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut ans = vec![];
    while 0 < n {
        ans.push(n % 5);
        n /= 5;
    }
    ans.reverse();
    println!("{}", ans.iter().map(|i| i * 2).join(""));
}
