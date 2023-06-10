use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m:usize,
        k:usize,
    }
    if m == 0 {
        println!("{}", if k == 0 { "0 0" } else { "-1" });
        return;
    }
    if m == 1 {
        println!("{}", if k == 0 { "0 0 1 1" } else { "-1" });
        return;
    }
    if (1 << m) <= k {
        println!("-1");
        return;
    }
    let mut ans = vec![];
    for i in 0..1 << m {
        if i != k {
            ans.push(i);
        }
    }
    ans.push(k);
    for i in (0..1 << m).rev() {
        if i != k {
            ans.push(i);
        }
    }
    ans.push(k);
    println!("{}", ans.iter().join(" "));
}
