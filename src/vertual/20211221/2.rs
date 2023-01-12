use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [i32; n],
    }
    let mut cnt:i32 = 0;

    for n in 0..pp.len()-2 {
        let mut vec = vec![pp[n], pp[n+1], pp[n+2]];
        vec.sort();
        if pp[n+1] == vec[1] {
            cnt = cnt+1;
        }
    }
    println!("{:?}", cnt);
}