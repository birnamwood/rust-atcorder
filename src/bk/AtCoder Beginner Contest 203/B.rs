use proconio::input;
 
fn main() {
    input!{
        n: i32,
        k: i32,
    }
    let mut ans:i32 = 0;
 
    for i in 0..n {
        for j in 0..k {
            ans = ans + ((i + 1) * 100 + (j + 1));
        }
    } 
 
    println!("{}", ans);
}