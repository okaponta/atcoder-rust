use itertools::{self, Itertools};
use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[u8;n],
    }
    a.insert(0, 0);
    let mut balls = vec![false; n + 1];
    let mut ans = vec![];
    for i in (1..=n).rev() {
        let mut tmp = i;
        let mut put = a[i] == 1;
        while tmp <= n {
            if balls[tmp] {
                put = !put;
            }
            tmp += i;
        }
        if put {
            ans.push(i);
            balls[i] = true;
        }
    }
    println!("{}", ans.len());
    if 0 < ans.len() {
        println!("{}", ans.iter().join(" "));
    }
}
