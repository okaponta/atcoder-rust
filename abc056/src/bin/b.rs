use proconio::input;

fn main() {
    input! {
        w:i64,
        a:i64,
        b:i64,
    }
    if a <= b && b <= a + w {
        println!("0");
        return;
    }
    if b <= a && a <= b + w {
        println!("0");
        return;
    }
    let mut ans = ((a + w) - b).abs();
    ans = ans.min((b + w) - a).abs();
    println!("{}", ans);
}
