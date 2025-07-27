fn main() {
    let x: i32 = 5; // Signed 32-bit integer use i8 if number is smaller and in range from -128 to 127
    let y: u32 = 10; // Unsigned 32-bit integer cannot be negative
    let z: f64 = 2.718; // 64-bit floating point number

    print!("x: {}, y: {}, z: {}", x, y, z); // Print variables without newline  
    println!(); // Print a newline at the end

    let is_male: bool = false; // Boolean variable, can be true or false
    let is_above_18: bool = true; // All variables are immutable by default
    // To make them mutable, use the mut keyword like this: let mut x: i32 = 5;

    if is_male {
        println!("You are male");
    } else {
        println!("You are gay");
    }

    if is_male && is_above_18 {
        println!("You are not gay"); //&& operator checks if both conditions are true
    }
}
