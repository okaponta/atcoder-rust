use proconio::input;

fn main() {
    input! {
        s:String
    }
    let one = s.chars().nth(0);
    let two = s.chars().nth(1);
    let three = s.chars().nth(2);
    println!(
        "{}",
        if one == two && one == three {
            "Won"
        } else {
            "Lost"
        }
    );
}
