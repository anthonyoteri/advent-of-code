/// Least Common Multiple
///
/// Calculate the least common multiple for a slice of numbers.
///
/// # Example:
///
/// ```
/// use aoc::math::lcm;
///
/// let nums = [2, 3, 4, 5, 6];
/// let result = lcm(&nums);
/// assert_eq!(60, result);
/// ```
pub fn lcm(input: &[i64]) -> i64 {
    if input.len() == 1 {
        return input[0];
    }
    let a = input[0];
    let b = lcm(&input[1..]);
    a * b / gcd(a, b)
}

/// Greatest Common Divisor
///
/// Calculate the greatest common divisor for two numbers.
///
/// # Example:
///
/// ```
/// use aoc::math::gcd;
///
/// let result = gcd(12, 18);
/// assert_eq!(6, result);
/// ```
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}