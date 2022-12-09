use proconio::input;

fn main() {
    input! {
        n:usize,
        t:i64,
        a:i64,
        h:[i64;n],
    }
    println!(
        "{}",
        h.into_iter()
            .enumerate()
            .map(|(i, h)| ((t * 1000 - h * 6 - a * 1000).abs(), i + 1))
            .min()
            .unwrap()
            .1
    );
}
