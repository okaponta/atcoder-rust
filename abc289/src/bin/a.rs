use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    for c in s {
        print!("{}", if c == '0' { '1' } else { '0' });
    }
    println!();
}
