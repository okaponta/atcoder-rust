use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut cur = 0;
    let mut ans = 0;
    let mut sum = 0;
    for a in a {
        cur += a;
        sum += a;
        if cur < 0 {
            ans += -cur;
            cur = 0;
        }
    }
    println!("{}", ans + sum);
}
