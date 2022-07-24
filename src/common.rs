pub const fn binomial(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

// TODO: Maybe use [std::num::NonZeroUsize] as the return type?
pub const fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub const fn pow(n: usize, mut k: usize) -> usize {
    let mut result = 1;
    while k > 0 {
        result *= n;
        k -= 1;
    }
    result
}

pub const fn even(n: usize) -> bool {
    n & 1 == 0
}

pub const fn odd(n: usize) -> bool {
    n & 1 != 0
}
