use proconio::input;

fn main() {
    input! {
        abc:[i32;3],
    }
    println!(
        "{}",
        if abc[0] <= abc[2] && abc[2] <= abc[1] {
            "Yes"
        } else {
            "No"
        }
    );
}
