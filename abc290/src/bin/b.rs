use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        k:usize,
        s:Chars,
    }
    let mut ans = vec![];
    let mut count = 0;
    for c in s {
        if c == 'o' && count < k {
            ans.push('o');
            count += 1;
        } else {
            ans.push('x');
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
