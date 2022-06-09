pub fn run(){
    let person:(&str, &str, i8) = ("Kelvin", "Keroka", 40);
    println!("{} is from {} and Kelvin is {}", person.0, person.1, person.2);
}