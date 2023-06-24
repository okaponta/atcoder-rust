use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let s = s[i].iter().chain(s[j].iter()).collect::<Vec<_>>();
            if is_kaibun(s) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn is_kaibun(c: Vec<&char>) -> bool {
    let n = c.len();
    for i in 0..n / 2 {
        if c[i] != c[n - 1 - i] {
            return false;
        }
    }
    true
}
