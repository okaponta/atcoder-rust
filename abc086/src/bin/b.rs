use proconio::input;

fn main() {
    input! {
        a:String,
        b:String,
    }
    let sq = (a + &b).parse::<usize>().unwrap();
    let mut tmp = 1;
    while tmp * tmp <= sq {
        if tmp * tmp == sq {
            println!("Yes");
            return;
        }
        tmp += 1;
    }
    println!("No");
}
