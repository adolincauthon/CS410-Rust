//! Modulus Exponent Calculator
//!
//! Adam Hiatt 2023

use std::env;

///Calculates the value of x^y mod m
/// 0 <= x < 2^64
/// 0 <= y < 2^64
/// 0 <  m < 2^64
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut base = x as u128;
    let mut exponent: u128 = y as u128;
    let modulus = m as u128;

    let mut result: u128 = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent /= 2;
        base = (base * base) % modulus;
    }
    u64::try_from(result).unwrap()
}

/// Tests the modexp function
#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
    let max = u64::max_value();
    assert_eq!(3, modexp(max, max, 6));
    assert_eq!(1, modexp(0, 0, 4))
}

///Parses a string into its u64 value
/// Exits program and prints erorr message if parsing is not successful
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

/// Prints error message with program usage and exits program
fn error() -> ! {
    eprintln!(
        "modexp: usage: modexp <x> <y> <m>\n - 0 <= x < 2^64 \n - 0 <= y < 2^64 \n - 0 <  m < 2^64"
    );
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error();
    }

    let base = parsenum(args[1].trim());
    let exponent = parsenum(args[2].trim());
    let modulus = parsenum(args[3].trim());
    if modulus == 0 {
        error();
    }

    let result = modexp(base, exponent, modulus);
    println!("{} ^ {} mod {} = {}", base, exponent, modulus, result);
}
