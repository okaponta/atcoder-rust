use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        a:[usize;q],
    }
    let mut flip = vec![0; n + 1];
    let mut ans = 0;
    for a in a {
        let bef = flip[a - 1] + flip[a];
        flip[a - 1] ^= 1;
        flip[a] ^= 1;
        let after = flip[a - 1] + flip[a];
        ans += (after - bef) / 2;
        println!("{}", ans);
    }
}
