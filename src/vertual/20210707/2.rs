use proconio::input;
 
fn main() {
    input!{
        s: String,
    }

    let cs: Vec<char> = s.chars().collect();

    if cs[2] == cs[3] {

        if cs[4] == cs[5] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}