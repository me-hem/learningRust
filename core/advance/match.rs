
enum Coin{
    Penny, Nickel, Dime, Quarter, 
}

fn main(){
    //Number game
    // let number = 0;

    // match number {
    //     1 => println!("The number is one!"),
    //     2 => println!("The number is two!"),
    //     3 => println!("The number is three!"),
    //     _ => println!("I Don't know"),
    // }

    let coin = Coin::Dime;

    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}