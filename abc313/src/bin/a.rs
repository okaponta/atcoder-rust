use proconio::input;

fn main() {
    input! {
        n:usize,
        mut p:[usize;n],
    }
    let base = p[0];
    p.sort();
    p.reverse();
    if p.len() == 1 {
        println!("0");
        return;
    }
    if p[0] == p[1] {
        if base == p[0] {
            println!("1");
        } else {
            println!("{}", p[0] + 1 - base);
        }
    } else {
        if base == p[0] {
            println!("0");
        } else {
            println!("{}", p[0] + 1 - base);
        }
    }
}
