use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars,
    }
    s.insert(0, 'E');
    s.push('W');
    let e = s.iter().filter(|&&c| c == 'E').count() - 1;
    let mut ans = e;
    let mut tmp = e;
    for i in 0..=n {
        if s[i] == 'W' {
            tmp += 1;
        }
        if s[i + 1] == 'E' {
            tmp -= 1;
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
