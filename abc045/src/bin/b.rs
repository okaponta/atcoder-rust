use proconio::{input, marker::Chars};

fn main() {
    input! {
        a:Chars,
        b:Chars,
        c:Chars,
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut next = 'a';
    loop {
        if next == 'a' {
            if i == a.len() {
                println!("A");
                return;
            }
            
            next = a[i];
            i += 1;
        } else if next == 'b' {
            if j == b.len() {
                println!("B");
                return;
            }
            
            next = b[j];
            j += 1;
        } else {
            if k == c.len() {
                println!("C");
                return;
            }
            
            next = c[k];
            k += 1;
        }
    }
}