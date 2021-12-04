use proconio::input;

fn main() {
    input! {
       n:i32, a:[i32;n]
    }
    let ans = (2..1001)
        .max_by_key(|&x| a.iter().filter(|&y| y % x == 0).count())
        .unwrap();
    println!("{}", ans);
}
