use proconio::input;

fn main() {
    input! {
        n:usize,
        l:i64,
        r:i64,
        a:[i64;n],
    }
    let mut sl = vec![0; n + 1];
    for i in 0..n {
        sl[i + 1] = (sl[i] + a[i]).min((i + 1) as i64 * l);
    }
    let mut sr = vec![0; n + 1];
    for i in (0..n).rev() {
        sr[i] = (sr[i + 1] + a[i]).min((n - i) as i64 * r);
    }
    println!(
        "{}",
        (0..=n).into_iter().map(|i| sl[i] + sr[i]).min().unwrap()
    );
}
