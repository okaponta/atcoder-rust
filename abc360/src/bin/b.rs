use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    for w in 1..s.len() {
        for i in 0..w {
            let mut u = vec![];
            for j in (i..s.len()).step_by(w) {
                u.push(s[j]);
            }
            if t == u {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
