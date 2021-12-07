use proconio::input;

fn main() {
    input! {
       n:i32,
    }
    let mut count = 0;
    for a in 1..n {
        count += (n - 1) / a;
    }
    println!("{}", count);
}
