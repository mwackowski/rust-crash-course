pub fn run() {
    let name = "Mateusz";
    let mut age = 30;
    println!("My name is {} and my age is {}", name, age); 

    age = 32;
    println!("My name is {} and my age is {}", name, age); 

    // Define constant
    const ID: i32 = 001;
    println!("id: {}", ID);

    // Assign multiple vars

    let (my_name, my_age) = ("Mateusz", 37);
    println!("{} is {}", my_name, my_age);

}