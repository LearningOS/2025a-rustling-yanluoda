// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.
// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // 使用迭代器计算阶乘
    // 对于 num = 0，范围 1..=0 是空的，所以 fold 返回初始值 1
    // 对于 num > 0，计算 1 * 2 * 3 * ... * num
    (1..=num).fold(1, |acc, x| acc * x)
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