use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut s:Chars,
        q:usize,
        txc:[(u8,usize,char);q],
    }
    let mut posi = 0;
    let mut flg = 1;
    for i in (0..q).rev() {
        if txc[i].0 != 1 {
            flg = txc[i].0;
            posi = i;
            break;
        }
    }
    for i in 0..n {
        if flg == 2 {
            s[i] = s[i].to_ascii_lowercase();
        } else if flg == 3 {
            s[i] = s[i].to_ascii_uppercase();
        }
    }
    for i in 0..q {
        if txc[i].0 == 1 {
            if i < posi {
                if flg == 1 {
                    s[txc[i].1 - 1] = txc[i].2;
                } else if flg == 2 {
                    s[txc[i].1 - 1] = txc[i].2.to_ascii_lowercase();
                } else {
                    s[txc[i].1 - 1] = txc[i].2.to_ascii_uppercase();
                }
            } else {
                s[txc[i].1 - 1] = txc[i].2;
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
