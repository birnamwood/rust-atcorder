use proconio::input;
 
fn main() {
    input!{
        n: usize,
        mut a: [i32; n],
    }
    let mut c = a.dedup();
    println!("{:?}", c);

    // for i in c.iter() {
    //     // ソートして1こずつ見る
    //     println!("{}", i)
    // }
}