pub fn run() {
    println!("FizzBuzz implementation: ");
    for n in 1..101 {
        // If n is divisible by 3 and 5, print FizzBuzz.
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        }
        // If n is divisible by 3, print Fizz.
        else if n % 3 == 0 {
            println!("Fizz");
        }
        // If n is divisible by 5, print Buzz.
        else if n % 5 == 0 {
            println!("Buzz");
        }
        // Otherwise, just print the number.
        else {
            println!("{}", n);
        }
    }
    println!();
}