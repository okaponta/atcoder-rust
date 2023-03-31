use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s:Chars,
        k:Usize1,
    }
    let mut hist: Vec<Vec<char>> = vec![];
    let mut best: Vec<String> = vec![];
    for c in s {
        for i in 0..hist.len() {
            hist[i].push(c);
            best.push(hist[i].iter().collect::<String>());
        }
        hist.push(vec![c]);
        hist.sort();
        hist.truncate(5);
        best.push(c.to_string());
        best.sort();
        best.dedup();
        best.truncate(5);
    }
    println!("{}", best[k]);
}
