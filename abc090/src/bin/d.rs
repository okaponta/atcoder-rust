use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    if k == 0 {
        println!("{}", n * n);
        return;
    }
    let mut ans = 0;
    for i in k + 1..=n {
        let tmp = ((n + 1) / i) * (i - k) + ((n + 1) % i).saturating_sub(k);
        ans += tmp;
    }
    println!("{}", ans);
}
