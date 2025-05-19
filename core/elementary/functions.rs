fn main(){
    add(10, 20);
    add(145, 40);
    add(36, 89);

    let x = 10;
    let y = 13;
    println!("Product of {} and {} is {}", x, y, mult(x, y));
}

// Explicit argument data type is mandatory
fn add(x: u32, y: u32){ 
    println!("Sum of {} and {} is {}", x, y, x+y);
}

//Returning values from function 
fn mult(x: u32, y: u32) -> u32{
    return x*y;
}