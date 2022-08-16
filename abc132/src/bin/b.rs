use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n],
    }
    println!(
        "{}",
        p.windows(3)
            .filter(|v| (v[0] < v[1] && v[1] < v[2]) || (v[2] < v[1] && v[1] < v[0]))
            .count()
    );
}
