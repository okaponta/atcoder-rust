use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
    }
    if s[0] == '<' && s[s.len() - 1] == '>' {
        s.pop();
        s.remove(0);
        if s.iter().all(|c| *c == '=') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
