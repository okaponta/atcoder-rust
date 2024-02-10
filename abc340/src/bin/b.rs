use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q:usize,
        ix:[(u8,usize);q],
    }
    let mut a = vec![];
    for (i, x) in ix {
        if i == 1 {
            a.push(x);
        } else {
            println!("{}", a[a.len() - x]);
        }
    }
}
