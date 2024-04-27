use proconio::input;

fn main() {
    input! {
        a:[usize;9],
        b:[usize;8],
    }
    let sa = a.into_iter().sum::<usize>();
    let sb = b.into_iter().sum::<usize>();
    println!("{}", sa - sb + 1);
}
