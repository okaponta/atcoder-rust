use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        d:usize,
        s:[Chars;n],
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..d {
        if (0..n).into_iter().all(|j| s[j][i] == 'o') {
            tmp += 1;
        } else {
            tmp = 0;
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
