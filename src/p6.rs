
fn main() {
    let suma: i32 = i32::pow((1..100).sum(),2);
    let sumb: i32 = (1..100).map(|x| x*x).sum();
    println!("{:?}", suma - sumb);
}
