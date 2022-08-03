use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n+1],
        mut b:[usize;n],
    }
    let bef = a.iter().sum::<usize>();
    for i in 0..n {
        let ai_bef = a[i];
        a[i] = a[i].saturating_sub(b[i]);
        b[i] -= ai_bef - a[i];
        a[i + 1] = a[i + 1].saturating_sub(b[i]);
    }
    println!("{}", bef - a.iter().sum::<usize>());
}
