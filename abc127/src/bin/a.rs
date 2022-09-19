use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize
    }
    println!(
        "{}",
        if 12 < a {
            b
        } else if 5 < a {
            b / 2
        } else {
            0
        }
    );
}
