use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        mut t:Chars
    }
    let b = t.drain(n..).rev().collect::<Vec<_>>();
    let zx = z(&t.iter().chain(b.iter()).collect::<Vec<_>>());
    let zy = z(&b.iter().chain(t.iter()).collect::<Vec<_>>());
    for i in 0..n {
        if zx[2 * n - i] == i && zy[n + i] == n - i {
            let ans = t[0..i]
                .iter()
                .chain(t[i..].iter().rev())
                .collect::<String>();
            println!("{}", ans);
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}

fn z(s: &Vec<&char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n + 1];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
