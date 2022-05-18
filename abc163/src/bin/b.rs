use proconio::input;

fn main() {
    input! {
        n:i64,
        m:i64,
        a:[i64;m],
    }
    let ans = n - a.iter().sum::<i64>();
    println!("{}", if ans < 0 { -1 } else { ans });
}
