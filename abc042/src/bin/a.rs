use proconio::input;

fn main() {
    input! {
        mut n:[usize;3],
    }
    n.sort();
    println!(
        "{}",
        if n[0] == 5 && n[1] == 5 && n[2] == 7 {
            "YES"
        } else {
            "NO"
        }
    );
}
