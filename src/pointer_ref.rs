pub fn run() {
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;
    
    println!("arr values: {:?}", (arr1, arr2));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("vec values: {:?}", (&vec1, vec2));
}