use proconio::input;

fn main() {
    input! {
        r:usize,
    }
    println!(
        "A{}C",
        if r < 1200 {
            "B"
        } else if r < 2800 {
            "R"
        } else {
            "G"
        }
    );
}
