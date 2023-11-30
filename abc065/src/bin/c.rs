use proconio::input;

fn main() {
    input! {
        mut n:[usize;2],
    }
    n.sort();
    let m = 1_000_000_007;
    let mut fact = vec![1];
    for i in 1..=n[1] {
        fact.push(fact[i - 1] * i % m);
    }
    if n[1] - n[0] == 0 {
        println!("{}", fact[n[0]] * fact[n[1]] * 2 % m);
    } else if n[1] - n[0] == 1 {
        println!("{}", fact[n[0]] * fact[n[1]] % m);
    } else {
        println!("0");
    }
}
