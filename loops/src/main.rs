fn main() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        print!("{} is even ", x);
    } else {
        print!("{} is odd", x);
    }

    for i in 0..10 {
        println!("{}", i);
    }
}

pub fn is_even(x:i32) -> bool {
    return x % 2 ==0;
}

