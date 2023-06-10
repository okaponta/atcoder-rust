use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut a = h;
    let mut b = 0;
    let mut c = w;
    let mut d = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                a = a.min(i);
                b = b.max(i);
                c = c.min(j);
                d = d.max(j);
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' && a <= i && i <= b && c <= j && j <= d {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
