use proconio::input;

fn main() {
    input! {
        s:String,_:String,
        mut a:usize,mut b:usize,
        u:String
    }
    if s == u {
        a -= 1;
    } else {
        b -= 1;
    }
    println!("{} {}", a, b);
}
