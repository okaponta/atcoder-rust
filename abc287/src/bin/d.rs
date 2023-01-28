use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let n = s.len();
    let m = t.len();
    let mut ss = vec![];
    for i in (0..m).rev() {
        ss.push(s[n - 1 - i]);
    }
    let mut count = 0;
    let mut is_ok = vec![false; m];
    for i in 0..m {
        if ss[i] == '?' || t[i] == '?' || ss[i] == t[i] {
            is_ok[i] = true;
            count += 1;
        }
    }
    println!("{}", if count == m { "Yes" } else { "No" });
    for i in 0..m {
        if is_ok[i] {
            count -= 1;
        }
        if s[i] == '?' || t[i] == '?' || s[i] == t[i] {
            is_ok[i] = true;
            count += 1;
        }
        println!("{}", if count == m { "Yes" } else { "No" });
    }
}
