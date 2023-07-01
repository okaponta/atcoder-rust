use proconio::input;

fn main() {
    input! {
        s:[usize;8],
    }
    if !(0..7).into_iter().all(|i| s[i] <= s[i + 1]) {
        println!("No");
        return;
    }
    if s[0] < 100 || 675 < s[7] {
        println!("No");
        return;
    }
    if !(0..8).into_iter().all(|i| s[i] % 25 == 0) {
        println!("No");
        return;
    }
    println!("Yes");
}
