use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut b = vec![vec![false; m]; n];
    for i in 0..n {
        input! {
            t:usize,
            a:[Usize1;t],
        }
        for a in a {
            b[i][a] = true;
        }
    }
    gaussian_elimination_xor(&mut b);
    input! {
        s: [u8; m],
    }
    let mut s = s.into_iter().map(|i| i == 1).collect::<Vec<_>>();
    for i in 0..m {
        if s[i] {
            for j in 0..n {
                if b[j][i] {
                    s = xor(&s, &b[j]);
                    break;
                }
            }
        }
    }
    if s.iter().any(|s| *s) {
        println!("{}", 0);
        return;
    }
    let zeros = b.iter().filter(|b| b.iter().all(|&b| !b)).count();
    println!("{}", pow(2, zeros, 998_244_353));
}

fn gaussian_elimination_xor(b: &mut Vec<Vec<bool>>) {
    let n = b.len();
    let m = b[0].len();

    let mut d = 0;
    for i in 0..m {
        for j in d..n {
            if b[j][i] {
                b.swap(d, j);
                break;
            }
        }

        if !b[d][i] {
            continue;
        }
        for j in 0..n {
            if j == d || !b[j][i] {
                continue;
            }
            for k in i..m {
                b[j][k] ^= b[d][k];
            }
        }
        d += 1;
        if d >= n {
            break;
        }
    }
}

fn xor(a: &[bool], b: &[bool]) -> Vec<bool> {
    std::iter::zip(a, b).map(|(&a, &b)| a ^ b).collect()
}

fn pow(mut x: usize, mut n: usize, modulo: usize) -> usize {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
