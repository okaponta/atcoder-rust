use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        hh:usize,
        ww:usize,
    }
    println!("{}", h * w + hh * ww - hh * w - h * ww);
}
