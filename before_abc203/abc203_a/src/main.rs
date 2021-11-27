use proconio::input;

fn main() {
    input! {
            a: i32,
            b: i32,
            c: i32,
    }
    let mut ans = 0;
    if a == b {
        ans = c;
    }
    if b == c {
        ans = a;
    }
    if c == a {
        ans = b;
    }
    println!("{}", ans)
}
