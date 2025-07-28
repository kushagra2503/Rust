fn main() {
    let mut x: String = String::from("Hi there"); //immuateble thats why we are using mut
    x.push_str(", my name is yash");
    println!("{}", x);
}
