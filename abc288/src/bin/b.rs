use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        s:[String;n],
    }
    let mut ans = vec![];
    for i in 0..k {
        ans.push(&s[i]);
    }
    ans.sort();
    for s in ans {
        println!("{}", s);
    }
}
