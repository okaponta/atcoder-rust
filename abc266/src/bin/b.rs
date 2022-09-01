use proconio::input;

fn main() {
    input! {
        n:i64,
    }
    println!("{}", n.rem_euclid(998244353));
}
