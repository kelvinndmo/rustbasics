// Variables are immutable by default and Rust is a block scoped language
pub fn run(){
    let name = "Kelvin Onkundi Ndemo";
    let mut age = 24;
    println!("My name is {} and i am {}", name, age);
    age = 40;
    println!("My name is {} and i am {}", name, age);

    //const keyword
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Asign multiple variables
    let (my_name, my_age) = ("Brad", 40);
    println!("{} and age is {}",my_name, my_age);
}