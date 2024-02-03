use proconio::input;

fn main() {
    input! {
        s:String,
    }
    let ss = s.split('.').collect::<Vec<_>>();
    println!("{}", ss[ss.len() - 1]);
}
