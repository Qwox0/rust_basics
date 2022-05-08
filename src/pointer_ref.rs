// Reference Pointer -> point to memory location

pub fn run() {
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Arrays: {:?}", (arr1, arr2));


    // non-primitive vector

    let vec1 = vec![1, 2, 3];
    println!("Vec1: {:?}", vec1);
    // let vec2 = vec1;  // -> vec1 will no longer hold the value (value is moved)
    let vec2 = &vec1;  // -> value is copied / borrowed
    println!("Vectors: {:?}", (&vec1, vec2));

}
