use proconio::input;

fn main() {
    input! {
       n:usize,
       ab:[(usize,usize);n],
    }
    let mut a = vec![];
    let mut b = vec![];
    for (ai, bi) in ab {
        a.push(ai);
        b.push(bi);
    }
    a.sort();
    b.sort();
    if n % 2 == 0 {
        println!("{}", 1 + b[n / 2] + b[n / 2 - 1] - a[n / 2] - a[n / 2 - 1]);
    } else {
        println!("{}", 1 + b[n / 2] - a[n / 2]);
    }
}
