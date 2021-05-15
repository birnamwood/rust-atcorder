use proconio::input;
 
fn main() {
    input!{
        n: usize,
        st: [(String, i32); n],
    }
    let mut a = vec![0;n];
    let mut b = vec![0;n];
    for (s, t) in st {
        if t != 0 {
            a.push(s);
            b.push(t);
        }
    }
    a.sort();
    let ln = a.len() -2;
    // println!("{:?}", a[ln]);

    for i in b {
        if b[i] == a[ln] {
            println!("{}", a[i]);
            break;
        }
    }

}