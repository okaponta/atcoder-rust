use proconio::input;

fn main() {
    input! {
       n:usize,
       h:[i32;n],
    }
    let mut ans = 0;
    for hi in h {
        if ans < hi {
            ans = hi;
        } else {
            println!("{}", ans);
            return;
        }
    }
    println!("{}", ans);
}
