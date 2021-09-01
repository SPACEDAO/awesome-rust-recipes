fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..99 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % (n++) == 0 {
            println!("fizz");
        } else if n % (n/2) == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
