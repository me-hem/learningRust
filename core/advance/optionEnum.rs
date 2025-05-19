fn main(){
    let numbers = vec![1, 2, 3, 4, 5];
    let index = find_index(numbers, 4);

    match index{
        Some(index) => println!("Element found at index: {}", index),
        None => println!{"Element not found!"}, 
    }
}


fn find_index(vec: Vec<i32>, element: i32) -> Option<usize> {
    for(index, value) in vec.iter().enumerate(){
        if *value == element{
            return Some(index);
        }
    
    }
    None
}