pub fn run() {
    let mut hello = String::from("hello ");

    println!("len: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");

    println!("capacity: {}", hello.capacity());
    println!("is empty: {}", hello.is_empty());
    println!("contains 'world': {}", hello.contains("world"));

    println!("replace: {}", hello.replace("World", "there"));
    for w in hello.split_whitespace() {
        println!("{}", w);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    assert_eq!(s, "ab");
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    

    println!("{}", hello);
}