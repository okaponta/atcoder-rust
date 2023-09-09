use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let div = divisor(n);
    let mut js = vec![];
    for j in div {
        if j < 10 {
            js.push(j);
        }
    }
    for i in 0..=n {
        let mut flg = false;
        for &j in &js {
            if i % (n / j) == 0 {
                flg = true;
                print!("{}", j);
                break;
            }
        }
        if !flg {
            print!("-")
        }
    }
    println!();
}

fn divisor(n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut upper = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}
