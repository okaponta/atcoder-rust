use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        t:Chars
    }
    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;
    for c in t {
        if c == 'S' {
            match dir % 4 {
                0 => x += 1,
                1 => y -= 1,
                2 => x -= 1,
                _ => y += 1,
            }
        } else {
            dir += 1;
        }
    }
    println!("{} {}", x, y);
}
