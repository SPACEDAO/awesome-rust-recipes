fn main() {
    // `n` will take the values: 1, 2, ..., 101 in each iteration
    for n in 1..101 {
        if n % (n+(n-1)) == 0 {
            println!("fizzbuzz");
        } else if (n-(n+1)) % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
