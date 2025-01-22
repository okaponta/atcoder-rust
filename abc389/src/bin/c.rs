use proconio::{marker::*, *};

fn main() {
    input! {
        q:usize,
    }
    let mut s = vec![0];
    let mut slide = 0;
    for _i in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {l:usize}
            s.push(s[s.len() - 1] + l);
        } else if qi == 2 {
            slide += 1;
        } else {
            input! {k:Usize1}
            println!("{}", s[k + slide] - s[slide]);
        }
    }
}
