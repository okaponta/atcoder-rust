use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        n:usize,
    }
    let l = s.len();
    let mut masks = vec![];
    let mut num = 0;
    for i in 0..l {
        if s[i] == '?' {
            masks.push(1 << (l - 1 - i));
        } else if s[i] == '1' {
            num += 1 << (l - 1 - i);
        }
    }
    if n < num {
        println!("-1");
        return;
    }
    for i in masks {
        if n < num + i {
            continue;
        }
        num += i;
    }
    println!("{}", num);
}
