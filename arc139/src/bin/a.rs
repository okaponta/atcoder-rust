use proconio::input;

fn main() {
    input! {
        n:usize,
        t:[i64;n],
    }
    let mut ans: i64 = 0;
    let mut prevt = -1;
    for ti in t {
        if ti > prevt {
            ans = ans >> ti;
            ans = ans << ti;
            ans += 1 << ti;
        } else if ti < prevt {
            ans += 1 << ti;
        } else {
            ans += 1 << (ti + 1);
        }
        if ans >> ti & 1 == 0 {
            ans += 1 << ti;
        }
        prevt = ti;
    }
    println!("{}", ans);
}
