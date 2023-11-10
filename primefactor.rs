fn main() {
    println!("start");
    let number: u64 = 600851475143;
    let mut primes: Vec<u64> = Vec::new();
    let mut i = findfirstprime(number);
    primes.push(i);
    let mut remainder = number;
    while i != 0 {
        println!("{i}");
        remainder = remainder / i;
        i = findfirstprime(remainder);
        primes.push(i);
    }
    for a in primes {
        println!("{a}");
    }
}

fn findfirstprime(startnumber: u64) -> u64 {
    for i in 2..=startnumber {
        if startnumber % i == 0 {
            return i;
        }
    }
    return 0;
}
