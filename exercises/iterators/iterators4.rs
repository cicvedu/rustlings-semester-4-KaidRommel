// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



struct FactNum {
    i: u64,
    n: u64,
    result: u64,
}

impl FactNum {
    fn new(x:u64) -> FactNum {
        FactNum {
            i: 1,
            n: x,
            result: 1
        }
    }
}

impl Iterator for FactNum {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.n {
            None
        } else {
            self.result *= self.i;
            self.i += 1;
            Some(self.result)
        }
    }
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    FactNum::new(num).last().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
