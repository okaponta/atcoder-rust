use proconio::{marker::*, *};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut sw = vec![];
    let mut r_sw = vec![];
    let mut ans = vec![];
    for i in 0..n {
        sw.push(i);
        r_sw.push(i);
        ans.push(i);
    }
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {a: Usize1, b:Usize1}
            ans[a] = r_sw[b];
        } else if qi == 2 {
            input! {a: Usize1, b:Usize1}
            r_sw.swap(a, b);
            sw.swap(r_sw[a], r_sw[b]);
        } else {
            input! {a: Usize1}
            println!("{}", sw[ans[a]] + 1);
        }
    }
}
