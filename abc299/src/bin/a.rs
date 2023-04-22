use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s: Chars,
    }
    let mut pipe = vec![];
    let mut aster = 0;
    for i in 0..n {
        if s[i] == '|' {
            pipe.push(i);
        }
        if s[i] == '*' {
            aster = i;
        }
    }
    pipe.sort();
    if pipe[0] < aster && aster < pipe[1] {
        println!("in");
    } else {
        println!("out");
    }
}
