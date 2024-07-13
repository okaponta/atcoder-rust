use proconio::{input, marker::Chars};

fn main() {
    input! {
        r:usize,
        g:usize,
        b:usize,
        c:Chars,
    }
    let ans = if c[0] == 'R' {
        g.min(b)
    } else if c[0] == 'G' {
        r.min(b)
    } else {
        r.min(g)
    };
    println!("{}", ans);
}
