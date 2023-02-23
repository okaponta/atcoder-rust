use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n:usize,
    }
    dfs(&mut vec!['a'; n], 0);
}

fn dfs(s: &mut Vec<char>, d: usize) {
    if d == s.len() {
        println!("{}", s.iter().collect::<String>());
        return;
    }
    s[d] = 'a';
    dfs(s, d + 1);
    s[d] = 'b';
    dfs(s, d + 1);
    s[d] = 'c';
    dfs(s, d + 1);
}
