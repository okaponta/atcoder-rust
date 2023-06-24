use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = vec![vec![]];
    ans[0].push('d');
    let mut idx = 0;
    for i in 0..n {
        if s[i] == '(' {
            ans.push(vec![]);
            idx += 1;
            ans[idx].push('(');
        } else if s[i] == ')' {
            if ans[idx][0] == '(' {
                ans.pop();
                idx -= 1;
            } else {
                ans.push(vec![]);
                idx += 1;
                ans[idx].push(')');
            }
        } else {
            ans[idx].push(s[i]);
        }
    }
    let mut s = vec![];
    for i in 0..ans.len() {
        for j in 0..ans[i].len() {
            s.push(ans[i][j]);
        }
    }
    s.remove(0);
    println!("{}", s.iter().collect::<String>());
}
