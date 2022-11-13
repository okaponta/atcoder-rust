use proconio::input;

fn main() {
    input! {
        n:usize,
        mut m:usize,
        mut ab:[(usize,usize);n],
    }
    ab.sort();
    let mut ans = 0;
    for (a, b) in ab {
        if m < b {
            ans += a * m;
            break;
        } else {
            ans += a * b;
            m -= b;
        }
    }
    println!("{}", ans);
}
