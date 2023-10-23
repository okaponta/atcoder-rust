use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut b = vec![0; 3];
    for a in a {
        if a % 2 == 1 {
            b[0] += 1;
        } else if a % 4 != 0 {
            b[1] += 1;
        } else {
            b[2] += 1;
        }
    }
    if b[1] == 0 {
        println!("{}", if n / 2 <= b[2] { "Yes" } else { "No" });
    } else {
        println!("{}", if n <= b[2] * 2 + b[1] { "Yes" } else { "No" });
    }
}
