use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        l:usize,
        r:usize,
    }
    let mut ans = vec![];
    for i in 1..l {
        ans.push(i);
    }
    for i in (l..=r).rev() {
        ans.push(i);
    }
    for i in r + 1..=n {
        ans.push(i)
    }
    println!("{}", ans.iter().join(" "));
}
