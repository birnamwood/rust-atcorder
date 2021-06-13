use proconio::input;
 
fn main() {
    input!{
        a: i32,
        b: i32,
    }
    let c = b as f32 /100.0 * a as f32;
    println!("{}", c);
}