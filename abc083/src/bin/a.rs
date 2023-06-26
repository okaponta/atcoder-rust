use proconio::input;

fn main() {
    input! {
        a:[usize;4],
    }
    let l = a[0] + a[1];
    let r = a[2] + a[3];
    println!(
        "{}",
        if l > r {
            "Left"
        } else if l < r {
            "Right"
        } else {
            "Balanced"
        }
    );
}
