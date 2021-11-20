use proconio::input;

fn main() {
    input! {
            n: i32
    }
    let list_price = 206;
    let price = n * 108 / 100;
    let mut ans = ":(";
    if list_price > price {
        ans = "Yay!"
    } else if list_price == price {
        ans = "so-so"
    }
    println!("{}", ans);
}
