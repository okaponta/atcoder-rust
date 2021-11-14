use proconio::input;

fn main() {
    input! {
            n: i32,
            mut st: [(String,i32);n]
    }
    st.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{}", st[1].0)
}
