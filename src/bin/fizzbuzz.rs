fn main() {
    for n in 1..=1000 {
        println!("{} => {}", n, check(n));
    }
}

fn check(n: usize) -> String {
    let dfive = n % 5 == 0;
    let dthree = n % 3 == 0;

    match (dfive, dthree) {
        (true, true) => String::from("FizzBuzz"),
        (false, true) => String::from("Fizz"),
        (true, false) => String::from("Buzz"),
        (false, false) => n.to_string(),
    }
}
