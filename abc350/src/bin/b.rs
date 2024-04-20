use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        t:[Usize1;q],
    }
    let mut ans = vec![true; n];
    for t in t {
        ans[t] = !ans[t];
    }
    println!("{}", ans.iter().filter(|i| **i).count());
}
