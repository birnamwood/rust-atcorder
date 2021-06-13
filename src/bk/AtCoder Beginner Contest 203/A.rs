use proconio::input;
 
fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    let mut ans: i32 = 0;
    if a != b && a != c && b != c {
        ans = 0;
    } else if a == b {
        ans = c;
    } else if a == c {
        ans = b;
    } else if b == c {
        ans = a;
    }
 
    println!("{}", ans);
}