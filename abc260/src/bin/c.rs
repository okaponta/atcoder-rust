use proconio::input;

fn main() {
    input! {
        n:usize,x:usize,y:usize,
    }
    let mut red = vec![0; 12];
    let mut blue = vec![0; 12];
    blue[0] = 1;
    for i in 0..11 {
        let b = red[i] + blue[i] * y;
        let r = b * x + red[i];
        red[i + 1] = r;
        blue[i + 1] = b;
    }
    println!("{}", red[n - 1]);
}
