use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        x:[i64;n],
    }
    let mut x1 = vec![1 << 60; n];
    let mut x2 = vec![1 << 60; n];
    for i in 0..n {
        if x[i] < 0 {
            x1[i] = -x[i];
        } else {
            x2[i] = x[i];
        }
    }
    x1.push(0);
    x2.push(0);
    x1.sort();
    x2.sort();
    let ans = (0..=k)
        .into_iter()
        .map(|i| (x1[i] * 2 + x2[k - i]).min(x1[i] + x2[k - i] * 2))
        .min()
        .unwrap();
    println!("{}", ans);
}
