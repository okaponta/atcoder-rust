use proconio::input;

fn main() {
    input! {
        a:i32,b:i32,c:i32,
    }
    let sub = a - b;
    let tkhs_win;
    if sub == 0 {
        tkhs_win = c == 1;
    } else {
        tkhs_win = sub > 0;
    }
    println!("{}", if tkhs_win { "Takahashi" } else { "Aoki" });
}
