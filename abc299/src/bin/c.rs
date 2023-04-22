use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars,
    }
    let mut count = 0i64;
    let mut tmp = 0;
    let mut flg = false;
    for i in 0..n {
        if s[i] == '-' {
            flg = true;
            tmp = 0;
        }
        if flg && s[i] == 'o' {
            tmp += 1;
            count = count.max(tmp);
        }
    }
    s.reverse();
    tmp = 0;
    for i in 0..n {
        if s[i] == '-' {
            flg = true;
            tmp = 0;
        }
        if flg && s[i] == 'o' {
            tmp += 1;
            count = count.max(tmp);
        }
    }
    if count == 0 {
        count = -1;
    }
    println!("{}", count);
}
