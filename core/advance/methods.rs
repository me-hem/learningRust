struct Rectangle{
    width: u32,
    height: u32,
}

// Methods only work on same type of argument
impl Rectangle{

    fn area(&self) -> u32 {
        self.width*self.height
    }
}


fn main(){
    let rect1 = Rectangle{
        width: 10,
        height: 9,
    };
    println!("Area = {}", rect1.area());
}