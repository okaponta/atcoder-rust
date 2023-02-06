use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let ans = a.into_iter().fold(0, |s, a| s + pow2(a));
    println!("{}", ans);
}

fn pow2(mut a: usize) -> usize {
    let mut res = 0;
    while a & 1 == 0 {
        a >>= 1;
        res += 1;
    }
    res
}
