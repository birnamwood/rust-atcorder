use proconio::input;
 
fn main() {
    input!{
        n: usize,
        a: [i32; n],
    }
    let mut count:i32 = 0;
    for i in 0..n-1 {
        for j in (i+1)..n {
            if (&a[j] - &a[i]) % 200 == 0 {
                count = count + 1;
            }
        }
    }
    println!("{}", count);
}