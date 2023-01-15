use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        s.into_iter()
            .fold(0, |s, c| s * 26 + (c as u8 - b'@') as usize)
    );
}
