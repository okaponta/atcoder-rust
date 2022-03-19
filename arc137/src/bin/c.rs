use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    if a[0] - a[1] > 1 {
        println!("Alice");
        return;
    }
    println!(
        "{}",
        if (a[0] + 1 - n) % 2 == 0 {
            "Bob"
        } else {
            "Alice"
        }
    );
}
