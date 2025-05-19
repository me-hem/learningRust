fn main(){
    let nums = vec![32, 43, 54, 65, 435, 534];
    println!("Largest number is {}", find_largest(&nums));

    let chars = vec!['a', 'b', 'a', 's', 'u', 'q', 'w'];
    println!("Largest character is {}", find_largest(&chars));

}

fn find_largest <T:std::cmp::PartialOrd> (nums: &[T]) -> &T{
    let mut largest = &nums[0];

    for num in nums{
        if num > largest{
            largest = num;
        }
    }

    largest

}