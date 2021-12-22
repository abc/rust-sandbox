pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

pub fn product(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for n in nums {
        if total == 0 {
            total = n;
        } else {
            total *= n;
        }
    }
    total
}

pub fn factorial(num: i32) -> i32 {
    product((0..=num).collect())
}

pub fn print_multiplication_tables() {
    for n in 1..13 {
        println!("{} times table", n);
        for x in 1..13 {
            let num = n * x;
            println!("{} × {}\t= {}", n, x, num);
        }
        println!();
    }
}

pub fn is_prime (num: i32) -> bool {
    for n in 2..num {
        if num % n == 0 {
            return false;
        }
    }

    true
}

pub fn run() {
    println!("Sum and product: ");
    let sum = sum((0..10).collect());
    let product = product((0..10).collect());
    println!("Sum: {} Product: {}", sum, product);
    println!();

    println!("Factorial: ");
    let fact = factorial(10);
    println!("10! = {}", fact);
    println!();

    println!("Multiplication tables: ");
    print_multiplication_tables();
    println!();

    println!("All square roots to 1000: ");
    for n in 1..1001 {
        if is_prime(n) {
            println!("{} is a prime number", n);
        }
    }
}