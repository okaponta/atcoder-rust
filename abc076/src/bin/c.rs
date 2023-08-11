use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let n = s.len();
    let m = t.len();
    if n < m {
        println!("UNRESTORABLE");
        return;
    }
    let mut str = vec![];
    'a: for i in 0..=n - m {
        let mut tmp = s.clone();
        for j in 0..m {
            if s[i + j] == '?' || s[i + j] == t[j] {
                tmp[i + j] = t[j];
            } else {
                continue 'a;
            }
        }
        str.push(tmp);
    }
    if str.len() == 0 {
        println!("UNRESTORABLE");
        return;
    }
    for i in 0..str.len() {
        for j in 0..n {
            if str[i][j] == '?' {
                str[i][j] = 'a';
            }
        }
    }
    str.sort();

    println!("{}", str[0].iter().collect::<String>());
}
