use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", if a.max(b) < 9 { "Yay!" } else { ":(" });
}
