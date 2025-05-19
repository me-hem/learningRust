fn main(){
    let x = 2734;
    let mut y = 2424;
    println!("The value of {} + {} is {}", x, y, x+y);

    // y = 12; by default variables are immutable

    y = 12; // we must declare the variable mutable if its values changes in the program
    println!("The value of {} + {} is {}", x, y, x+y);


    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Value of const is {THREE_HOURS_IN_SECONDS}")

}