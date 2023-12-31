//! Toy RSA
//! Adam Hiatt
//! 10-18-23

use toy_rsa_lib::*;

/// Fixed RSA encryption exponent
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let mut x = rsa_prime();
    let mut y = rsa_prime();
    let mut least_mult = lcm(u64::from(x - 1), u64::from(y - 1));
    let mut greatest_cd = gcd(EXP, least_mult);
    while EXP > least_mult && greatest_cd != 1 {
        x = rsa_prime();
        y = rsa_prime();
        least_mult = lcm(u64::from(x - 1), u64::from(y - 1));
        greatest_cd = gcd(EXP, least_mult);
    }
    (x, y)
}

/// Encrypt the plaintext `msg` using the RSAlet mut  public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(u64::from(msg), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let key_prod: u64 = u64::from(key.0) * u64::from(key.1);
    let d = modinverse(EXP, lcm(u64::from(key.0 - 1), u64::from(key.1 - 1)));
    modexp(msg, d, key_prod)
        .try_into()
        .unwrap_or_else(|_| error("Received bad value for encryption"))
}

///print error message and exit
fn error(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1)
}
