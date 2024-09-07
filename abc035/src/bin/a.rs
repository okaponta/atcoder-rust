use proconio::input;

fn main() {
    input! {
       mut w:usize,
       mut h:usize,
    }
    if (w / 4) * 3 == h {
        println!("4:3");
    } else {
        println!("16:9");
    }
}
