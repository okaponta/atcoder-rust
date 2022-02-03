use proconio::input;

fn main() {
    input! {
        rc:[(i64,i64);2],
    }
    let x = (rc[0].0 - rc[1].0).abs();
    let y = (rc[0].1 - rc[1].1).abs();
    println!(
        "{}",
        if x == 0 && y == 0 {
            0
        } else if x == y || x + y <= 3 {
            1
        } else if (x + y) % 2 == 0 || (x - y).abs() <= 3 || x + y <= 6 {
            2
        } else {
            3
        }
    );
}
