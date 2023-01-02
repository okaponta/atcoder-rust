use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut used = vec![false; 26];
    let mut tmp = vec![vec![false; 26]];
    let mut lev = 0;
    for c in s {
        if c == '(' {
            lev += 1;
            if tmp.len() == lev {
                tmp.push(vec![false; 26]);
            }
        } else if c == ')' {
            for i in 0..26 {
                if tmp[lev][i] {
                    used[i] = false;
                    tmp[lev][i] = false;
                }
            }
            lev -= 1;
        } else {
            let idx = (c as u8 - b'a') as usize;
            if used[idx] {
                println!("No");
                return;
            } else {
                used[idx] = true;
                tmp[lev][idx] = true;
            }
        }
    }
    println!("Yes");
}
