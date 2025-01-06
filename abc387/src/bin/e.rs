use proconio::{marker::*, *};

fn main() {
    input! {
        n:Chars,
    }
    let l = n.len();
    if l < 6 {
        let mut m = 0;
        for i in 0..l {
            m += f(n[l - 1 - i]) * 10usize.pow(i as u32);
        }
        g(m);
        return;
    }
    let x = f(n[0]) * 10 + f(n[1]);
    match x {
        ..=16 => print!("17"),
        17..=25 => print!("26"),
        26..=43 => print!("44"),
        44..=79 => print!("80"),
        _ => print!("125"),
    }
    (2..l).into_iter().for_each(|_| print!("0"));
    println!();
}

fn f(c: char) -> usize {
    c.to_digit(10).unwrap() as usize
}

fn g(n: usize) {
    for i in n..2 * n {
        if i % h(i) == 0 && (i + 1) % h(i + 1) == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}

fn h(mut n: usize) -> usize {
    let mut res = 0;
    while 0 < n {
        res += n % 10;
        n /= 10;
    }
    res
}
