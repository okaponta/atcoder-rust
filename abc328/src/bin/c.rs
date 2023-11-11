use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        s:Chars,
        lr:[(Usize1,Usize1);q],
    }
    let mut sum = vec![0; n];
    for i in 1..n {
        if s[i - 1] == s[i] {
            sum[i] = sum[i - 1] + 1;
        } else {
            sum[i] = sum[i - 1]
        }
    }
    for (l, r) in lr {
        println!("{}", sum[r] - sum[l]);
    }
}
