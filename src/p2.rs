
fn main() {
    let mut xs: [i32; 2] = [1,1];
    let mut sum: u64 = 0;
    loop {
        let hold = xs[0];
        xs[0] = xs[0] + xs[1];
        if xs[0] % 2 == 0 {
            sum += xs[0] as u64;
        }
        xs[1] = hold;
        if xs[0] > 4000000 {
            break;
        }
    }
    println!("{:?}", sum);
}
