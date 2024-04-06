use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[[u16;m];n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if judge(&a[i], &a[j], m) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn judge(a: &Vec<u16>, b: &Vec<u16>, m: usize) -> bool {
    let mut res = false;
    for k in 0..m {
        res ^= a[k] == b[k];
    }
    res
}
