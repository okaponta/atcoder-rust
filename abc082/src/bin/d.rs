use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
        x:i64,
        y:i64,
    }
    s.push('T');
    let mut x_prob = HashSet::new();
    let mut y_prob = HashSet::new();
    y_prob.insert(0);
    let mut first = true;
    let mut is_x = true;
    let mut tmp = 0;
    for c in s {
        if c == 'F' {
            tmp += 1;
        }
        if c == 'T' {
            if is_x {
                if first {
                    x_prob.insert(tmp);
                    first = false;
                } else {
                    let mut new = HashSet::new();
                    for i in x_prob {
                        new.insert(i + tmp);
                        new.insert(i - tmp);
                    }
                    x_prob = new;
                }
            } else {
                let mut new = HashSet::new();
                for i in y_prob {
                    new.insert(i + tmp);
                    new.insert(i - tmp);
                }
                y_prob = new;
            }
            is_x = !is_x;
            tmp = 0;
        }
    }
    if x_prob.contains(&x) && y_prob.contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
