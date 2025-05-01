use proconio::{marker::*, *};

fn main() {
    input! {n:usize,m:usize,ab:[(Usize1,Usize1);m]}
    let mut c = vec![0; n];
    for (a, b) in ab {
        c[(a + b) % n] += 1;
    }
    let mut ans = 0;
    for c in c {
        ans += (m - c) * c;
    }
    println!("{}", ans / 2);
}
