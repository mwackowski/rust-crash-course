use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[2] = 22;
    println!("{:?}", numbers);

    println!("second value: {:?}", numbers[1]);

    println!("array length {}", numbers.len());
    println!("this array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("slice: {:?}", slice);
}