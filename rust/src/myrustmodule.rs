pub fn factorial(a: i64) -> i64 {
        let mut x = 1;
        for i in 1..(a+1) {
            x *= i;
        }
        x
    }

