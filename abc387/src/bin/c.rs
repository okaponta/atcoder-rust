use proconio::{marker::*, *};

fn main() {
    input! {
        l:Usize1,
        r:usize,
    }
    println!("{}", f(r) - f(l));
}

fn f(mut n: usize) -> usize {
    let mut s = vec![];
    while 0 < n {
        s.push(n % 10);
        n /= 10;
    }
    s.reverse();
    let m = s.len() as u32;
    let mut res = 0;
    for i in 1usize..10 {
        if s[0] < i {
            res += g(0, m - 1, i);
        } else if s[0] == i {
            if (1..m as usize).all(|j| s[j] < i) {
                res += 1;
            }
            for j in 1..m {
                if s[j as usize] < i {
                    res += s[j as usize] * i.pow(m - j as u32 - 1);
                } else {
                    res += i.pow(m - j as u32);
                    break;
                }
            }
            res += g(0, m - 1, i);
        } else {
            res += g(0, m, i);
        }
    }
    res
}

fn g(l: u32, r: u32, b: usize) -> usize {
    (l..r).into_iter().map(|i| b.pow(i)).sum::<usize>()
}
