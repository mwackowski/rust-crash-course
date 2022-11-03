use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers[2] = 22;
    numbers.push(99);
    numbers.pop();
    
    println!("{:?}", numbers);
    println!("second value: {:?}", numbers[1]);
    println!("vector length {}", numbers.len());
    println!("this vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("slice: {:?}", slice);

    for x in numbers.iter() {
        println!("number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("numbers vec: {:?}", numbers);
}