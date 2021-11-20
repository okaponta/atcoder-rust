use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       h: usize,
       w: usize,
       x: usize,
       y: usize,
       s: [Chars;h],
    }
    // self+2
    let mut count = -1;
    // upper
    for i in (0..x).rev() {
        if s[i][y - 1] == '#' {
            break;
        }
        count += 1;
    }
    // lower
    for i in x..h {
        if s[i][y - 1] == '#' {
            break;
        }
        count += 1;
    }
    // right
    for i in y..w {
        if s[x - 1][i] == '#' {
            break;
        }
        count += 1;
    }
    // left
    for i in (0..y).rev() {
        if s[x - 1][i] == '#' {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}
