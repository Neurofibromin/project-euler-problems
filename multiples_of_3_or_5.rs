fn main() {
    println!("program start");
    let aim = 1000;
    println!("{aim}");
    let mut counter = 1;
    let mut sum = 0;
    while counter < aim {
        if counter % 5 == 0 {
            sum += counter;
        } else if counter % 3 == 0 {
            sum += counter;
        }
        counter += 1;
    }

    print!("{sum}")
}
