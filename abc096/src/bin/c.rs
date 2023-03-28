use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        mut s:[Chars;h],
    }
    s.insert(0, vec!['.'; w + 2]);
    s.push(vec!['.'; w + 2]);
    for i in 1..=h {
        s[i].insert(0, '.');
        s[i].push('.');
    }
    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '.' {
                continue;
            }
            if s[i - 1][j] == '.' && s[i][j - 1] == '.' && s[i + 1][j] == '.' && s[i][j + 1] == '.'
            {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
