fn main(){
    //Integers
    let integer123: i32 = 42; //Signed 32-bit integer
    let unsigned_int: u64 = 10_000; //Unsigned 64-bit integer

    println!("Integer: {}", integer123);
    println!("unsigned integer: {}", unsigned_int);


    //Floating-point numbers
    let float: f64 = 3.14; //64-bit floating-point number

    println!("Float: {}", float);

    //Booleans
    let is_true: bool = true;
    let is_false: bool = false;

    println!("is_true: {}", is_true);
    println!("is_false: {}", is_false);

    //Characters
    let char1: char = 'A';
    let char2: char = '1';

    println!("Char1: {}", char1);
    println!("Char2: {}", char2);

    //Compound types - Primitive (Tuple, Array)
    let tuple: (i32, f64, char) = (10, 3.1, 'A');
    let array = ["Today", "Yesterday", "Tomorrow"];
    
    println!("Tuple[0]: {}", tuple.0);
    println!("Array[1]: {}", array[1]);

}