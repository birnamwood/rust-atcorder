use proconio::input;
 
fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    let d: i32;

    if a > b {
        if b > c {
            d = a + b;    
        } else {
            d = a + c;
        }
    } else {
        if a > c {
            d = b + a;
        } else {
            d = b + c;
        }
    }
    println!("{}", d);
}