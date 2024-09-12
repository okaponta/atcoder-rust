use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
       t:u8,
    }
    let mut x = 0i64;
    let mut y = 0i64;
    let mut q = 0;
    for c in s {
        if c == 'U' {
            x += 1;
        } else if c == 'D' {
            x -= 1;
        } else if c == 'L' {
            y -= 1;
        } else if c == 'R' {
            y += 1;
        } else {
            q += 1;
        }
    }
    let d = x.abs() + y.abs();
    if t == 1 {
        println!("{}", d + q);
    } else {
        if q < d {
            println!("{}", d - q);
        } else {
            println!("{}", (q - d) % 2);
        }
    }
}
