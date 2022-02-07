use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    let mut count = vec![0; 10];
    for c in &s {
        count[c.to_digit(10).unwrap() as usize] += 1;
    }
    let mut two = vec![];
    let mut tmp = 0;
    while tmp < 100 {
        tmp += 8;
        let mut c = vec![0; 10];
        for d in tmp.to_string().chars() {
            c[d.to_digit(10).unwrap() as usize] += 1;
        }
        if c[0] > 0 {
            continue;
        }
        two.push(c);
    }
    if s.len() < 3 {
        if two.contains(&count) {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }
    let mut three = vec![];
    while tmp < 1000 {
        tmp += 8;
        let mut c = vec![0; 10];
        for d in tmp.to_string().chars() {
            c[d.to_digit(10).unwrap() as usize] += 1;
        }
        if c[0] > 0 {
            continue;
        }
        three.push(c);
    }
    for e in three {
        let mut flag = true;
        for i in 1..10 {
            if e[i] > count[i] {
                flag = false;
            }
        }
        if flag {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
