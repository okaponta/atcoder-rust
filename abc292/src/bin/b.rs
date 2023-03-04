use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        ix:[(u8,Usize1);q],
    }
    let mut count = vec![0; n];
    for (i, x) in ix {
        if i == 1 {
            count[x] += 1;
        }
        if i == 2 {
            count[x] += 2;
        }
        if i == 3 {
            println!("{}", if count[x] > 1 { "Yes" } else { "No" });
        }
    }
}
