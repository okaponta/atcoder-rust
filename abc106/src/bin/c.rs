use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        k:usize,
    }
    let mut one = 0;
    let mut num = '0';
    for i in 0..s.len() {
        if s[i] == '1' {
            one += 1;
        } else {
            num = s[i];
            break;
        }
    }
    if k <= one {
        println!("1");
    } else {
        println!("{}", num);
    }
}
