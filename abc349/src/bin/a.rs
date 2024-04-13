use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n-1],
    }
    let s = a.into_iter().sum::<i64>();
    println!("{}", -s);
}
