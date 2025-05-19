//Custom Data Types

#[derive(Debug)]

//Structure 
struct User{
    active: bool,
    username: String,
    gender: Gender,
    email: String,
    sign_in_count: u8,
}


//Struture cant be use if we have just a value
// struct IpAddr{
//     v4: String,
//     v6: ,
// }
// let my_ip = IpAddr{
//     v4: String::from("127.0.0.1"),
//     v6: //novalue,
// }

#[derive(Debug)]
//Enum Data Type
enum Gender{
    Male(u8),
    Female(u8),
    Other(u8),
}

fn main(){

    let umesh = build_user(String::from("temp@gmail.com"), String::from("umeshbhatt"));

    println!("{:#?}", umesh);
    println!("Username: {:#?\n\n}", umesh.username);


    let umesh2 = User{
        email: String::from("temp2@gmail.com"),
        ..umesh
    };
    println!("{:#?}", umesh2);
    println!("Username: {:#?}", umesh2.username);
    println!("Gender: {:#?}", umesh2.gender);

}

fn build_user(email: String, username: String) -> User {

    User{
        active: true,
        username,
        email,
        gender: Gender::Male(20),
        sign_in_count: 1,
    }
}