fn main() {
    stack_fn(); // Calling the function that uses stack memory
    heap_fn(); //Calling the function that uses heap memory
    update_string(); //Calling the function that changes size of variables t runtime
}

fn stack_fn() {
    //Declaring few integers on stack
    let a = 10;
    let b = 20;
    let c = a+b;
    println!("Stack function: The sum of {} and {} is {}", a,b,c)
}

fn heap_fn() {
    //Creating a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1,s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    //Start with a base string on heap
    let mut s = String::from("initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length:{}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    //Append some text to the string
    s.push_str("and some additional text");
    println!("Capacity: {}, Length:{}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    println!("After update: {}", s);
}
