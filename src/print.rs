pub fn run() {
    // print
    println!("hello from print.rs file");

    // basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Mateusz", "Warsaw");

    // positional arguments
    println!("{0} is from {1} and likes to {2}", "Mateusz", "Warsaw", "code");

    // named arguments
    println!("{name} likes to play {activity}", name="John", activity="baseball");

    // placeholder traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10+10); 
}