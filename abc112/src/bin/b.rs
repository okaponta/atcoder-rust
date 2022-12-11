use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        ct:[(usize,usize);n],
    }
    println!(
        "{}",
        if let Some(c) = ct.into_iter().filter(|ct| ct.1 <= t).min() {
            c.0.to_string()
        } else {
            "TLE".to_string()
        }
    );
}
