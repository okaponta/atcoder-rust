use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        h:[usize;n],
    }
    let s = ruiseki(&h);
    for i in 0..=n {
        if m < s[i] {
            println!("{}", i - 1);
            return;
        }
    }
    println!("{}", n);
}

fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
