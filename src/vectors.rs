
pub fn run(){
    let mut numbers : Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);

    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}