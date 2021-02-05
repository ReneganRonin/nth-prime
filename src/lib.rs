pub fn nth(n: u32) -> u32 {
    println!("What is the 0-indexed {}th prime number?", n);
    nth_prime(n)
}

pub fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            // a prime number should have a remainder of 1
            return false;
        }
    }
    true
}

pub fn nth_prime(n: u32) -> u32 {
    let prime: u32 = 2;
    if n + 1 == 1 {
        // index starts at 0 so plus 1
        return 2;
    }
    if n + 1 > 1 {
        let mut prime: u32 = 2;
        let mut num: u32 = 3;
        while prime < n + 1 {
            num += 2;
            prime += match is_prime(num) {
                true => 1,
                false => 0,
            };
        }
        num
    } else {
        prime + 1 // if n + 1 equals 2, while loop "auto" exits and then proceeds to else block?
                  // Since prime was not added because while loop exited,
                  // it is safe to assume that we should return prime + 1 or 3
    }
}
