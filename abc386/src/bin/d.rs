use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut xyc:[(Usize1,Usize1,char);m],
    }
    xyc.sort();
    let mut tmp = n;
    for (_x, y, c) in xyc {
        if c == 'B' {
            if tmp <= y {
                println!("No");
                return;
            }
        } else {
            tmp = tmp.min(y);
        }
    }
    println!("Yes");
}
