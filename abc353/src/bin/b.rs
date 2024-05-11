use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    let mut tmp = 0;
    for a in a {
        tmp += a;
        if k < tmp {
            ans += 1;
            tmp = a;
        }
    }
    println!("{}", ans + 1);
}
