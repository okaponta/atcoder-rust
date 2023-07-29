use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut tmp = 1;
    while (tmp + 1) * (tmp + 1) <= n {
        tmp += 1;
    }
    println!("{}", tmp * tmp);
}
