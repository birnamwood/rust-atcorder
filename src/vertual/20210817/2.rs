use proconio::input;

fn main() {
    input!{
        a: String,
    }
    let ans: String = a.replace(",", " ");


    println!("{}", ans);
}