use proconio::*;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    let mut imos = vec![0; 1000002];
    for (a, b) in ab {
        imos[a] += 1;
        imos[b + 1] -= 1;
    }
    for i in 1..1000002 {
        imos[i] += imos[i - 1];
    }
    println!("{}", imos.iter().max().unwrap());
}
