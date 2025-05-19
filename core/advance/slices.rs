fn main(){
    let s = String::from("hello world");
    println!("{}", first_word(&s));
}


fn first_word(s: &String) -> &str { //Usize is dynmic data type for unsigned int
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    println!("{}", b' ');
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}