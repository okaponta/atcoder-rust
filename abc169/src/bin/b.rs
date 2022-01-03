use proconio::input;

const MAX: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
       n:usize,
       mut a:[i64;n],
    }
    if a.iter().any(|e| e == &0) {
        println!("{}", 0);
        return;
    }
    let mut ans = 1;
    for ai in a {
        if ans > MAX / ai {
            println!("{}", -1);
            return;
        }
        ans *= ai;
    }
    println!("{}", ans);
}
