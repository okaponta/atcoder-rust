use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let max = n * (n - 1) / 2;
    if max + 1 - n < k {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if k + ans.len() == max {
                break;
            }
            ans.push((i + 1, j + 1));
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{} {}", ans[i].0, ans[i].1);
    }
}
