use proconio::input;

fn main() {
    input! {
        n:usize,
        xu:[(f64,String);n],
    }
    println!(
        "{}",
        xu.iter().fold(0.0, |s, (x, u)| {
            s + if u == "JPY" { *x } else { x * 380000.0 }
        })
    );
}
