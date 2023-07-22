use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        d:usize,
        s:[Chars;n],
    }
    let mut ok = vec![];
    for i in 0..d {
        if (0..n).into_iter().all(|j| s[j][i] == 'o') {
            ok.push(i);
        }
    }
    if ok.len() == 0 {
        println!("0");
        return;
    }
    let mut ans = 1;
    let mut tmp = 1;
    for i in 1..ok.len() {
        if ok[i] == ok[i - 1] + 1 {
            tmp += 1;
        } else {
            tmp = 1;
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
