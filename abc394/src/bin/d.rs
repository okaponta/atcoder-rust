use proconio::{marker::*, *};

fn main() {
    input! {
        s:Chars,
    }
    let mut v = vec![];
    let br = vec![('(', ')'), ('[', ']'), ('<', '>')];
    for c in s {
        v.push(c);
        if v.len() < 2 {
            continue;
        }
        let c1 = v[v.len() - 2];
        let c2 = v[v.len() - 1];
        if br.contains(&(c1, c2)) {
            v.pop();
            v.pop();
        }
    }
    println!("{}", if v.len() == 0 { "Yes" } else { "No" });
}
