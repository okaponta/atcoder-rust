use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ta = 0;
    let mut ao = 0;
    for i in 0..n {
        if s[i] == 'T' {
            ta += 1;
        } else {
            ao += 1;
        }
    }
    if ta == ao {
        println!("{}", if s[n - 1] == 'T' { 'A' } else { 'T' });
        return;
    }
    println!("{}", if ao < ta { 'T' } else { 'A' });
}
