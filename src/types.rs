pub fn run(){
    //Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // add explicit type
    let z: i64 = 454545454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    // Get Boolean from an expression.
    let is_greater = 10 > 5;

    //Character
    let a1 = '😊';
    let face = '\u{1F604}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1,face))

   
}