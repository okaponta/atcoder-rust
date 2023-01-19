use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        if s[0] != 'A'
            || (2..s.len() - 1)
                .into_iter()
                .filter(|&i| s[i] == 'C')
                .count()
                != 1
            || (0..s.len())
                .into_iter()
                .filter(|&i| b'a' <= s[i] as u8 && s[i] as u8 <= b'z')
                .count()
                != s.len() - 2
        {
            "WA"
        } else {
            "AC"
        }
    )
}
