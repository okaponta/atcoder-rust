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
    let mut ac = vec![0; n];
    for i in 1..n {
        ac[i] = ac[i - 1];
        if s[i - 1] == 'A' && s[i] == 'C' {
            ac[i] += 1;
        }
    }
    for (l, r) in lr {
        println!("{}", ac[r] - ac[l]);
    }
}
