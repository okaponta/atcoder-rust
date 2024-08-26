use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let max = *a.iter().max().unwrap();
    let min = *a.iter().min().unwrap();
    if 0 < max.abs() && min.abs() <= max.abs() {
        let i = a.iter().position(|&x| x == max).unwrap();
        f(n, &a, i);
    } else {
        let i = a.iter().position(|&x| x == min).unwrap();
        g(n, &a, i);
    }
}

fn f(n: usize, a: &Vec<i64>, j: usize) {
    let mut ans = vec![];
    for i in 0..n {
        if a[i] < 0 {
            ans.push((j, i));
        }
    }
    for i in 0..n - 1 {
        ans.push((i, i + 1));
    }
    p(ans);
}

fn g(n: usize, a: &Vec<i64>, j: usize) {
    let mut ans = vec![];
    for i in 0..n {
        if 0 < a[i] {
            ans.push((j, i));
        }
    }
    for i in (0..n - 1).rev() {
        ans.push((i + 1, i));
    }
    p(ans);
}

fn p(ans: Vec<(usize, usize)>) {
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
