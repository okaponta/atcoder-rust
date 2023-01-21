use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        mut s:Chars,
    }
    let mut i = 0;
    loop {
        if s.len() - 1 <= i {
            break;
        }
        if s[i] == 'n' && s[i + 1] == 'a' {
            s.insert(i + 1, 'y');
            i += 2;
        } else {
            i += 1;
        }
    }
    println!("{}", s.iter().collect::<String>());
}
