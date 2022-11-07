use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        c:i64,
        b:[i64;m],
        a:[[i64;m];n],
    }
    println!(
        "{}",
        a.into_iter()
            .filter(|a| 0 < (0..m).into_iter().fold(c, |s, i| s + a[i] * b[i]))
            .count()
    );
}
