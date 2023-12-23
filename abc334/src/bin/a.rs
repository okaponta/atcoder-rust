use proconio::input;

fn main() {
    input! {
        b:usize,
        g:usize,
    }
    println!("{}", if b > g { "Bat" } else { "Glove" });
}
