fn main(){
    //If-else
    let num = 54;

    if num % 2 == 0 {
        println!("Number is even.");
    } else{
        println!("Number is odd.");
    }

    //Iteration: loop, while, for
    let mut x = 0;
    
    //loop
    loop {
        x+=1;
        println!("Value of x is {}", x);

        if x > 5 {
            break;
        }
    }

    //while-loop
    x = 0;
    while x <=5 {
        x+=1;
        println!("Value of x is {}", x);
    }

    //for-loop
    let a = [1, 2, 3, 4, 5];
    for ele in a{
        println!("Value of x is {}", ele);
    }

    for num in 1..6{
        println!("Value of x is {}", num);
    }

    for num in (1..6).rev(){
        println!("Reverse value of x is {}", num);
    }
}