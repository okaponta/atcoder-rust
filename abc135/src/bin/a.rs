use proconio::input;

fn main() {
    input! {
        a:i64,
        b:i64,
    }
    let diff = (a - b).abs();
    if diff % 2 != 0 {
        println!("IMPOSSIBLE");
    } else {
        println!("{}", (a + b) / 2);
    }
}
