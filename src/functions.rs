pub fn run() {
    greeting("hello", "Doe");
    
    let get_sum = add(5, 5);
    println!("sum: {}", get_sum);

    let n3: i32 = 13;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}