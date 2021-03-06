pub fn collatz(arg: u128) -> u128{
    let mut n = arg;

    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };

        println!("{}", n);
    }

    n
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(1, collatz(9999));
    }

    #[test]
    fn it_works_2() {
        assert_ne!(2, collatz(9999));
    }
}
