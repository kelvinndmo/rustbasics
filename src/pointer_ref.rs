pub fn run(){
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    //vectors -> Not primitive
    let numbers = vec![1,2,3];
    let num2 = &numbers;

    println!("{:?}", (&numbers, num2))
}