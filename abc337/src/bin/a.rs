use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let s = xy.into_iter().fold(0, |s, i| s + i.0 - i.1);
    println!(
        "{}",
        if s < 0 {
            "Aoki"
        } else if s == 0 {
            "Draw"
        } else {
            "Takahashi"
        }
    );
}
