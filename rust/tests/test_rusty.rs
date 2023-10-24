extern crate rusty;

#[cfg(test)]
mod tests {

    #[test]
    fn test_true() {
        assert!(true);
    }

    #[test]
    fn test_factorial() {
        assert!(rusty::myrustmodule::factorial(5) == 120);
    }

}



