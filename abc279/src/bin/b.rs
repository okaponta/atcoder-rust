use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let sl = s.len();
    let tl = t.len();
    if sl < tl {
        println!("No");
        return;
    }
    for i in 0..=sl - tl {
        if t.iter().enumerate().all(|(j, c)| c == &s[i + j]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
