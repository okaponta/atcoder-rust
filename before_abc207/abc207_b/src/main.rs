use proconio::input;

fn main() {
    input! {
            a: i64,
            b: i64,
            c: i64,
            d: i64
    }
    let mut ans = 0;
    let mut red = 0;
    let mut blue = a;
    if b >= c * d {
        ans = -1
    } else {
        while blue > d * red {
            red += c;
            blue += b;
            ans += 1;
        }
    }
    println!("{}", ans);
}
