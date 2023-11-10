fn main() {
    println!("Hello, world!");
    let mut list: Vec<i32> = Vec::new();
    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let number = i * j;
            let reverse = reverse(&number.to_string());
            if number.to_string() == reverse {
                println!("{number}");
                list.push(number);
            }
            //println!("{number}");
        }
    }
    list.sort();
    println!("Largest ");
    match list.last() {
        Some(x) => println!("Result : {x}"),
        None => println!("Not found"),
    };
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
