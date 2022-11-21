use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}",
        if s[5] == '0' && vec!['1', '2', '3', '4'].contains(&s[6]) {
            "Heisei"
        } else {
            "TBD"
        }
    );
}
