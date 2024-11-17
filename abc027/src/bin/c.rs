use proconio::*;

fn main() {
    input! {mut n:usize}
    n += 1;
    let mut ans = true;
    while 1 < n {
        if ans {
            n += 1;
        }
        n /= 2;
        ans = !ans;
    }
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
