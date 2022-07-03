use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        q:usize,
        s:Chars,
        query:[(usize,usize);q],
    }
    let mut rot = 0;
    for (t, x) in query {
        if t == 1 {
            rot += n - x;
            rot %= n;
        } else {
            println!("{}", s[(rot + x - 1) % n]);
        }
    }
}
