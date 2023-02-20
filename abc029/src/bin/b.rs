use proconio::input;

fn main() {
    input! {
       s:[String;12],
    }
    println!("{}", s.iter().filter(|s| s.contains('r')).count());
}
