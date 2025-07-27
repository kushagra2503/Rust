fn main() {
    let greeting: String = String::from("Hello, world!");
    println!("{}", greeting);

    let char1 : Option<char> = greeting.chars().nth(1);
    match char1 {
        Some(c) => println!("Character at index : {}", c), //{} = 1 [nth()]
        None => println!("No character found at index 10000"),
    }
}
