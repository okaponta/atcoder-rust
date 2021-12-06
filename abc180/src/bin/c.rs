use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
       n:i64,
    }
    let mut ans = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            ans.push(i);
            ans.push(n / i);
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans.iter().join("\n"));
}
