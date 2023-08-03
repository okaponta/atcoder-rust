use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
        c:i64,
    }
    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
