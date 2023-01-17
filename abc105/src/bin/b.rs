use proconio::input;

const NG: [usize; 9] = [1, 2, 3, 5, 6, 9, 10, 13, 17];

fn main() {
    input! {n:usize}
    println!("{}", if NG.contains(&n) { "No" } else { "Yes" });
}
