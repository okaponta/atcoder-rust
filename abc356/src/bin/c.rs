use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    let mut ok = vec![true; 1 << n];
    for _ in 0..m {
        input! {
            c:usize,
            a:[Usize1;c],
            r:char,
        }
        for i in 0..1 << n {
            let mut count = 0;
            for &j in &a {
                if i >> j & 1 == 1 {
                    count += 1;
                }
            }
            if r == 'o' {
                if count < k {
                    ok[i] = false;
                }
            } else {
                if k <= count {
                    ok[i] = false;
                }
            }
        }
    }
    println!("{}", ok.into_iter().filter(|b| *b).count());
}
