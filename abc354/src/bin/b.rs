use proconio::input;

fn main() {
    input! {
        n:usize,
        mut sc:[(String, usize);n],
    }
    sc.sort();
    let ord = sc.iter().map(|(_, c)| c).sum::<usize>() % n;
    println!("{}", sc[ord].0);
}
