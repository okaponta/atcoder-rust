use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
       l:Usize1,r:Usize1,
       mut s:Chars,
    }
    let num = (r - l + 1) / 2;
    for i in 0..num {
        s.swap(l + i, r - i);
    }
    println!("{}", s.iter().collect::<String>());
}
