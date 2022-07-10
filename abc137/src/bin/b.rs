use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k:i32,
        x:i32,
    }
    let mut ans = vec![];
    for i in x - k + 1..x + k {
        ans.push(i);
    }
    println!("{}", ans.iter().join(" "));
}
