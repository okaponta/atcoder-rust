use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }
    let mut is_t = false;
    if a < c {
        is_t = true;
    } else if a == c {
        if b <= d {
            is_t = true;
        }
    }
    println!("{}", if is_t { "Takahashi" } else { "Aoki" });
}
