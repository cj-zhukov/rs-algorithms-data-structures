// A prime number is a whole number greater than 1 with exactly two divisors: 1 and itself.
// For example, 2 is a prime number because it is only divisible by 1 and 2.
// In contrast, 4 is not prime since it is divisible by 1, 2 and 4.

// Rewrite sumPrimes so it returns the sum of all prime numbers that are less than or equal to num.

pub fn sum_primes(num: i64) -> i64 {
    (2..=num).filter(|&n| is_prime(n)).sum()
}

// Function to find out if a number is a prime by checking if the given number (n) is NOT evenly
// dividable by all numbers from 2 to the square root of the given number (n)
fn is_prime(n: i64) -> bool {
    (2..=(n as f64).sqrt() as i64).all(|i| n % i != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(sum_primes(10), 17);
    }

    #[test]
    fn test2() {
        assert_eq!(sum_primes(977), 73156);
    }
}