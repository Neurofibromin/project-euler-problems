fn main() {
    println!("started");
    let aim = 4000000;
    //let aim = 100;
    let mut number1 = 1;
    let mut number2 = 2;
    let mut number3 = 0;
    let mut sum = 2;
    while number3 < aim {
        number3 = number2 + number1;
        number1 = number2;
        number2 = number3;

        println!("{number3}");
        if number3 % 2 == 0 {
            sum += number3;
        }
    }
    println!("Finalsum:{sum}");
}
