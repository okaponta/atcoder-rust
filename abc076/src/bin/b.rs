use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let mut tmp = 1;
    for _ in 0..n {
        tmp = (tmp * 2).min(tmp + k);
    }
    println!("{}", tmp);
}
