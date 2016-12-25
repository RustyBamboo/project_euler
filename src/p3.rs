fn find_factors(mut x: u64) {
    while x % 2 == 0 {
        println!("{:?}", 2);
        x /= 2;
    }
    let mut i = 3;
    while i as f64 <= (x as f64).sqrt() {
        i+=2;
        while x%i == 0 {
            println!("{:?}", i);
            x /= i;
        }
    }

    if x > 2 {
        println!("{:?}", x);
    }
}

fn main() {
    find_factors(600851475143)
}
