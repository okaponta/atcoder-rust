use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let mut ans = vec![k / 50; 50];
    for i in 0..50 {
        if i < k % 50 {
            ans[i] += 50;
        } else {
            ans[i] += 49 - k % 50;
        }
    }
    println!("50");
    println!("{}", ans.iter().join(" "));
}
