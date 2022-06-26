use proconio::input;

fn main() {
    input! {
        n:usize,x:usize,y:usize,
        a:[usize;n],
    }
    let mut flag = false;
    let mut t_count = 0;
    let mut a_count = 0;
    for ai in a {
        if x <= ai % (x + y) {
            t_count += 1;
            flag = true;
        } else {
            t_count -= 1;
        }
        if y <= ai % (x + y) {
            a_count += 1;
        } else {
            a_count -= 1;
        }
    }
    if a_count < t_count {
        println!("First");
        return;
    }
    if a_count == t_count {
        if flag {
            println!("First");
            return;
        }
    }
    println!("Second");
}
