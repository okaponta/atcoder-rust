use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[usize;n],
        b:[usize;m],
    }
    a.sort();
    for bi in b {
        let res = a.binary_search(&bi);
        match res {
            Ok(index) => {
                a.remove(index);
            }
            Err(_) => {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
