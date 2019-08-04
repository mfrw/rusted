extern crate primal;
use primal::Sieve;

pub fn primes() {
    let sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));

    let not_a_prime = 1024;
    println!(
        "{} is not prime: {}",
        not_a_prime,
        sieve.is_prime(not_a_prime)
    );

    let n = 1000;
    match sieve.primes_from(0).nth(n - 1) {
        Some(number) => println!("{}th prime is: {}", n, number),
        None => println!("I dont know about {}th prime", n),
    }
    println!("{:?}", num_divisors(2610, &sieve));
}

fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}
