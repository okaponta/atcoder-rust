use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize, mut x:u128,
        s:Chars
    }
    let mut s_cmp = vec![];
    let mut count = 0;
    for i in (0..n).rev() {
        if s[i] == 'U' {
            count += 1;
        } else {
            if count > 0 {
                count -= 1;
            } else {
                s_cmp.push(s[i]);
            }
        }
    }
    for _ in 0..count {
        s_cmp.push('U');
    }
    for c in s_cmp.iter().rev() {
        match c {
            'U' => x /= 2,
            'L' => x = 2 * x,
            _ => x = 2 * x + 1,
        }
    }
    println!("{}", x);
}
