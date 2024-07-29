use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = vec![];
    for i in 0..n {
        ans.push((a[i], i + 1));
    }
    ans.sort();
    ans.reverse();
    for i in 0..n {
        println!("{}", ans[i].1);
    }
}
