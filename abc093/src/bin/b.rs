use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize,
        b:usize,
        k:usize,
    }
    let mut ans = vec![];
    for i in 0..k {
        ans.push(a + i);
        ans.push(b.saturating_sub(i));
    }
    ans.sort();
    ans.dedup();
    for ans in ans {
        if a <= ans && ans <= b {
            println!("{}", ans);
        }
    }
}
