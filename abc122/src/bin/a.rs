use proconio::input;

fn main() {
    input! {
        c:char,
    }
    println!(
        "{}",
        match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            _ => 'A',
        }
    );
}
