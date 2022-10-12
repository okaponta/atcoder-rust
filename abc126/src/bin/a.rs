use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _:usize,
        k:Usize1,
        mut s:Chars
    }
    s[k] = s[k].to_ascii_lowercase();
    println!("{}", s.iter().collect::<String>());
}
