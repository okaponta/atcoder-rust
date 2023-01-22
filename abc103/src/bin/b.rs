use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
        t:String,
    }
    let n = s.len();
    for _ in 0..n {
        if s.iter().collect::<String>() == t {
            println!("Yes");
            return;
        }
        s.push(s[0]);
        s.remove(0);
    }
    println!("No");
}
