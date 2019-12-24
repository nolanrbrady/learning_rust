pub fn fizz_buzz() {
    for n in 1..100 {
        if n % 2 == 0 {
            println!("Fizz");
        } else if n % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
        break;
    }
}
