use std::process::exit;

pub fn collatz(arg: u128) -> u128 {
    match arg {
        0 => exit(-1),
        1 => arg,
        n if n % 2 == 0 => collatz(n / 2),
        _ => collatz(3 * arg + 1),
    }
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
